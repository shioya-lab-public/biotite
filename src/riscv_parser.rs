use crate::riscv_isa::*;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::mem;
use std::str::Lines;

lazy_static! {
    static ref SECTION_SIZE: Regex =
        Regex::new(r"\s+\S+\s+(\S+)\s+([[:xdigit:]]+)\s+([[:xdigit:]]+)").unwrap();
    static ref SYMBOL_SIZE: Regex =
        Regex::new(r"([[:xdigit:]]+)\s+\S+\s+\S+\s+\S+\s+([[:xdigit:]]+)\s+(\S+)").unwrap();
    static ref SECTION: Regex = Regex::new(r"Disassembly of section (\S+):").unwrap();
    static ref SYMBOL: Regex = Regex::new(r"([[:xdigit:]]+) <(\S+)>:").unwrap();
    static ref BYTES: Regex =
        Regex::new(r"\s+[[:xdigit:]]+:\s+(([[:xdigit:]][[:xdigit:]] )+)").unwrap();
}

pub fn parse(src: &str) -> Program {
    let mut src = src.lines();
    let entry = parse_entry(&mut src);
    let sections = parse_sections(&mut src);
    let symbols = parse_symbols(&mut src);
    let (mut data_blocks, code_blocks) = parse_assembly(&mut src);
    expand_data_blocks(&mut data_blocks, &sections, &symbols);
    Program {
        entry,
        data_blocks,
        code_blocks,
    }
}

fn parse_entry(src: &mut Lines) -> Addr {
    let mut src = src.skip(3);
    Addr::new(
        src.next()
            .expect("Missing file header")
            .strip_prefix("start address: 0x")
            .expect("Invalid file header"),
    )
}

fn parse_sections(src: &mut Lines) -> HashMap<(String, Addr), usize> {
    let mut src = src.skip(4);
    let mut sections = HashMap::new();
    while let Some(caps) = SECTION_SIZE.captures(src.next().expect("Missing section header")) {
        let section = (String::from(&caps[1]), Addr::new(&caps[3]));
        let size = usize::from_str_radix(&caps[2], 16).unwrap();
        sections.insert(section, size);
    }
    sections
}

fn parse_symbols(src: &mut Lines) -> HashMap<(String, Addr), usize> {
    let mut src = src.skip(1);
    let mut symbols = HashMap::new();
    while let Some(caps) = SYMBOL_SIZE.captures(src.next().expect("Missing symbol table")) {
        let symbol = (String::from(&caps[3]), Addr::new(&caps[1]));
        let size = usize::from_str_radix(&caps[2], 16).unwrap();
        symbols.insert(symbol, size);
    }
    symbols
}

fn parse_assembly(src: &mut Lines) -> (Vec<DataBlock>, Vec<CodeBlock>) {
    let mut data_blocks = Vec::new();
    let mut code_blocks = Vec::new();
    let mut section = String::new();
    let mut symbol = String::new();
    let mut address = Addr(0);
    let mut lines = Vec::new();
    for line in src.filter(|l| !l.is_empty()) {
        if let Some(caps) = SYMBOL.captures(line) {
            let sym = caps[2].to_string();
            let addr = Addr::new(&caps[1]);
            let raw_block = (
                section.clone(),
                mem::replace(&mut symbol, sym),
                mem::replace(&mut address, addr),
                mem::take(&mut lines),
            );
            if raw_block.3.is_empty() {
                continue;
            } else if raw_block.0 == ".text" {
                code_blocks.push(raw_block);
            } else {
                data_blocks.push(raw_block);
            }
        } else if let Some(caps) = SECTION.captures(line) {
            let sec = caps[1].to_string();
            let raw_block = (
                mem::replace(&mut section, sec),
                mem::take(&mut symbol),
                address,
                mem::take(&mut lines),
            );
            if raw_block.3.is_empty() {
                continue;
            } else if raw_block.0 == ".text" {
                code_blocks.push(raw_block);
            } else {
                data_blocks.push(raw_block);
            }
        } else {
            lines.push(line);
        }
    }
    let raw_block = (section, symbol, address, lines);
    if raw_block.0 == ".text" {
        code_blocks.push(raw_block);
    } else {
        data_blocks.push(raw_block);
    }
    (
        data_blocks.into_par_iter().map(parse_data_block).collect(),
        code_blocks.into_par_iter().map(parse_code_block).collect(),
    )
}

fn parse_data_block(
    (section, symbol, address, lines): (String, String, Addr, Vec<&str>),
) -> DataBlock {
    let mut bytes = Vec::new();
    if lines.len() > 1 || lines[0] != "..." {
        for line in lines {
            let caps = BYTES.captures(line).unwrap();
            for byte_str in caps[1].split(' ').filter(|s| !s.is_empty()) {
                let byte =
                    u8::from_str_radix(byte_str, 16).unwrap_or_else(|_| panic!("Invalid byte `{byte_str}`"));
                bytes.push(byte);
            }
        }
    }
    DataBlock {
        section,
        symbol,
        address,
        bytes,
    }
}

fn parse_code_block(
    (section, symbol, address, lines): (String, String, Addr, Vec<&str>),
) -> CodeBlock {
    CodeBlock {
        section,
        symbol,
        address,
        insts: lines.into_iter().map(Inst::new).collect(),
    }
}

fn expand_data_blocks(
    data_blocks: &mut Vec<DataBlock>,
    sections: &HashMap<(String, Addr), usize>,
    symbols: &HashMap<(String, Addr), usize>,
) {
    for data_block in data_blocks {
        if data_block.bytes.is_empty() {
            let symbol = data_block.symbol.clone();
            let address = data_block.address;
            let size = match symbols.get(&(symbol, address)) {
                Some(size) => *size,
                None => {
                    let section = data_block.section.clone();
                    match sections.get(&(section, address)) {
                        Some(usize) => *usize,
                        None => unreachable!(),
                    }
                }
            };
            data_block.bytes = vec![0; size];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::riscv_isa::*;
    use FReg::*;
    use Inst::*;
    use Reg::*;
    use CSR::*;
    use MO::*;
    use RM::*;

    macro_rules! build_tests {
        ( $( $func:ident ( $src:literal, $inst:tt { $( $field:ident: $value:expr ),* } ), )* ) => {
            $(
                #[test]
                fn $func() {
                    let src = concat!("
test:	file format elf64-littleriscv
architecture: riscv64
start address: 0x0000000000010528

Sections:
Idx Name                Size     VMA              Type
  0                     00000000 0000000000000000 

SYMBOL TABLE:

Disassembly of section .text:

0000000000010528 <_start>:
   10528: ef 00 e0 02  	", $src);
                    let prog = super::parse(src);
                    assert_eq!(
                        prog,
                        Program {
                            entry: Addr(0x10528),
                            data_blocks: Vec::new(),
                            code_blocks: vec![
                                CodeBlock {
                                    section: String::from(".text"),
                                    symbol: String::from("_start"),
                                    address: Addr(0x10528),
                                    insts: vec![
                                        $inst {
                                            address: Addr(0x10528),
                                            is_compressed: false,
                                            $(
                                                $field: $value,
                                            )*
                                        }
                                    ]
                                }
                            ]
                        }
                    );
                }
            )*
        };
    }

    #[test]
    fn program() {
        let src = "
test:	file format elf64-littleriscv
architecture: riscv64
start address: 0x0000000000010528

Sections:
Idx Name                Size     VMA              Type
  0                     00000000 0000000000000000 
  21 .bss                000011d8 0000000000070e40 BSS

SYMBOL TABLE:
0000000000070a78 g     O .sbss	0000000000000008 _dl_tls_generation

Disassembly of section .bss:

0000000000070e40 <.bss>:
...

Disassembly of section .sbss:

0000000000070a78 <_dl_tls_generation>:
...

Disassembly of section .text:

00000000000103b0 <abort>:
   103b0: 4d 71        	addi	sp, sp, -336

0000000000010528 <_start>:
   10528: ef 00 e0 02  	jal	0x10556 <load_gp>

Disassembly of section .rodata:

000000000004c370 <.rodata>:
   4c370: 48 65        	ld	a0, 136(a0)

000000000004c430 <__PRETTY_FUNCTION__.0>:
   4c430: 5f 5f 6c 69  	<unknown>
";
        let prog = super::parse(src);
        assert_eq!(
            prog,
            Program {
                entry: Addr(0x10528),
                data_blocks: vec![
                    DataBlock {
                        section: String::from(".bss"),
                        symbol: String::from(".bss"),
                        address: Addr(0x70e40),
                        bytes: vec![0; 0x11d8],
                    },
                    DataBlock {
                        section: String::from(".sbss"),
                        symbol: String::from("_dl_tls_generation"),
                        address: Addr(0x70a78),
                        bytes: vec![0; 0x8],
                    },
                    DataBlock {
                        section: String::from(".rodata"),
                        symbol: String::from(".rodata"),
                        address: Addr(0x4c370),
                        bytes: vec![0x48, 0x65],
                    },
                    DataBlock {
                        section: String::from(".rodata"),
                        symbol: String::from("__PRETTY_FUNCTION__.0"),
                        address: Addr(0x4c430),
                        bytes: vec![0x5f, 0x5f, 0x6c, 0x69],
                    },
                ],
                code_blocks: vec![
                    CodeBlock {
                        section: String::from(".text"),
                        symbol: String::from("abort"),
                        address: Addr(0x103b0),
                        insts: vec![Inst::Addi {
                            address: Addr(0x103b0),
                            is_compressed: true,
                            rd: Sp,
                            rs1: Sp,
                            imm: Imm(-336),
                        }]
                    },
                    CodeBlock {
                        section: String::from(".text"),
                        symbol: String::from("_start"),
                        address: Addr(0x10528),
                        insts: vec![Inst::PseudoJal {
                            address: Addr(0x10528),
                            is_compressed: false,
                            addr: Addr(0x10556),
                        }]
                    },
                ]
            }
        );
    }

    build_tests! {
        // Registers
        x0_f0("flw	ft0, 0(zero)", Flw { frd: Ft0, imm: Imm(0), rs1: Zero }),
        x1_f1("flw	ft1, 0(ra)", Flw { frd: Ft1, imm: Imm(0), rs1: Ra }),
        x2_f2("flw	ft2, 0(sp)", Flw { frd: Ft2, imm: Imm(0), rs1: Sp }),
        x3_f3("flw	ft3, 0(gp)", Flw { frd: Ft3, imm: Imm(0), rs1: Gp }),
        x4_f4("flw	ft4, 0(tp)", Flw { frd: Ft4, imm: Imm(0), rs1: Tp }),
        x5_f5("flw	ft5, 0(t0)", Flw { frd: Ft5, imm: Imm(0), rs1: T0 }),
        x6_f6("flw	ft6, 0(t1)", Flw { frd: Ft6, imm: Imm(0), rs1: T1 }),
        x7_f7("flw	ft7, 0(t2)", Flw { frd: Ft7, imm: Imm(0), rs1: T2 }),
        x8_f8("flw	fs0, 0(s0)", Flw { frd: Fs0, imm: Imm(0), rs1: S0 }),
        x9_f9("flw	fs1, 0(s1)", Flw { frd: Fs1, imm: Imm(0), rs1: S1 }),
        x10_f10("flw	fa0, 0(a0)", Flw { frd: Fa0, imm: Imm(0), rs1: A0 }),
        x11_f11("flw	fa1, 0(a1)", Flw { frd: Fa1, imm: Imm(0), rs1: A1 }),
        x12_f12("flw	fa2, 0(a2)", Flw { frd: Fa2, imm: Imm(0), rs1: A2 }),
        x13_f13("flw	fa3, 0(a3)", Flw { frd: Fa3, imm: Imm(0), rs1: A3 }),
        x14_f14("flw	fa4, 0(a4)", Flw { frd: Fa4, imm: Imm(0), rs1: A4 }),
        x15_f15("flw	fa5, 0(a5)", Flw { frd: Fa5, imm: Imm(0), rs1: A5 }),
        x16_f16("flw	fa6, 0(a6)", Flw { frd: Fa6, imm: Imm(0), rs1: A6 }),
        x17_f17("flw	fa7, 0(a7)", Flw { frd: Fa7, imm: Imm(0), rs1: A7 }),
        x18_f18("flw	fs2, 0(s2)", Flw { frd: Fs2, imm: Imm(0), rs1: S2 }),
        x19_f19("flw	fs3, 0(s3)", Flw { frd: Fs3, imm: Imm(0), rs1: S3 }),
        x20_f20("flw	fs4, 0(s4)", Flw { frd: Fs4, imm: Imm(0), rs1: S4 }),
        x21_f21("flw	fs5, 0(s5)", Flw { frd: Fs5, imm: Imm(0), rs1: S5 }),
        x22_f22("flw	fs6, 0(s6)", Flw { frd: Fs6, imm: Imm(0), rs1: S6 }),
        x23_f23("flw	fs7, 0(s7)", Flw { frd: Fs7, imm: Imm(0), rs1: S7 }),
        x24_f24("flw	fs8, 0(s8)", Flw { frd: Fs8, imm: Imm(0), rs1: S8 }),
        x25_f25("flw	fs9, 0(s9)", Flw { frd: Fs9, imm: Imm(0), rs1: S9 }),
        x26_f26("flw	fs10, 0(s10)", Flw { frd: Fs10, imm: Imm(0), rs1: S10 }),
        x27_f27("flw	fs11, 0(s11)", Flw { frd: Fs11, imm: Imm(0), rs1: S11 }),
        x28_f28("flw	ft8, 0(t3)", Flw { frd: Ft8, imm: Imm(0), rs1: T3 }),
        x29_f29("flw	ft9, 0(t4)", Flw { frd: Ft9, imm: Imm(0), rs1: T4 }),
        x30_f30("flw	ft10, 0(t5)", Flw { frd: Ft10, imm: Imm(0), rs1: T5 }),
        x31_f31("flw	ft11, 0(t6)", Flw { frd: Ft11, imm: Imm(0), rs1: T6 }),

        // // RV32I (42 tests)
        // lui("lui t0,4", Lui { rd: T0, imm: Immediate(4) }),
        // auipc("auipc t0,4", Auipc { rd: T0, imm: Immediate(4) }),
        // jal("jal ra,main", Jal { rd: Ra, addr: Address(0x0) }),
        // jalr("jalr t0,4(t0)", Jalr { rd: T0, imm: Immediate(4), rs1: T0 }),
        // implicit_jalr("jalr 4(t0)", ImplicitJalr { imm: Immediate(4), rs1: T0 }),
        // beq("beq t0,t1,main", Beq { rs1: T0, rs2: T1, addr: Address(0x0) }),
        // bne("bne t0,t1,main", Bne { rs1: T0, rs2: T1, addr: Address(0x0) }),
        // blt("blt t0,t1,main", Blt { rs1: T0, rs2: T1, addr: Address(0x0) }),
        // bge("bge t0,t1,main", Bge { rs1: T0, rs2: T1, addr: Address(0x0) }),
        // bltu("bltu t0,t1,main", Bltu { rs1: T0, rs2: T1, addr: Address(0x0) }),
        // bgeu("bgeu t0,t1,main", Bgeu { rs1: T0, rs2: T1, addr: Address(0x0) }),
        // lb("lb t0,-4(t1)", Lb { rd: T0, imm: Immediate(-4), rs1: T1 }),
        // lh("lh t0,-4(t1)", Lh { rd: T0, imm: Immediate(-4), rs1: T1 }),
        // lw("lw t0,-4(t1)", Lw { rd: T0, imm: Immediate(-4), rs1: T1 }),
        // lbu("lbu t0,-4(t1)", Lbu { rd: T0, imm: Immediate(-4), rs1: T1 }),
        // lhu("lhu t0,-4(t1)", Lhu { rd: T0, imm: Immediate(-4), rs1: T1 }),
        // sb("sb t0,-4(t1)", Sb { rs2: T0, imm: Immediate(-4), rs1: T1 }),
        // sh("sh t0,-4(t1)", Sh { rs2: T0, imm: Immediate(-4), rs1: T1 }),
        // sw("sw t0,-4(t1)", Sw { rs2: T0, imm: Immediate(-4), rs1: T1 }),
        // addi("addi t0,t1,4", Addi { rd: T0, rs1: T1, imm: Immediate(4) }),
        // slti("slti t0,t1,4", Slti { rd: T0, rs1: T1, imm: Immediate(4) }),
        // sltiu("sltiu t0,t1,4", Sltiu { rd: T0, rs1: T1, imm: Immediate(4) }),
        // xori("xori t0,t1,4", Xori { rd: T0, rs1: T1, imm:Immediate(4) }),
        // ori("ori t0,t1,4", Ori { rd: T0, rs1: T1, imm: Immediate(4) }),
        // andi("andi t0,t1,4", Andi { rd: T0, rs1: T1, imm: Immediate(4) }),
        // slli("slli t0,t1,4", Slli { rd: T0, rs1: T1, imm: Immediate(4) }),
        // srli("srli t0,t1,4", Srli { rd: T0, rs1: T1, imm: Immediate(4) }),
        // srai("srai t0,t1,4", Srai { rd: T0, rs1: T1, imm: Immediate(4) }),
        // add("add t0,t1,t2", Add { rd: T0, rs1: T1, rs2: T2 }),
        // sub("sub t0,t1,t2", Sub { rd: T0, rs1: T1, rs2: T2 }),
        // sll("sll t0,t1,t2", Sll { rd: T0, rs1: T1, rs2: T2 }),
        // slt("slt t0,t1,t2", Slt { rd: T0, rs1: T1, rs2: T2 }),
        // sltu("sltu t0,t1,t2", Sltu { rd: T0, rs1: T1, rs2: T2 }),
        // xor("xor t0,t1,t2", Xor { rd: T0, rs1: T1, rs2: T2 }),
        // srl("srl t0,t1,t2", Srl { rd: T0, rs1: T1, rs2: T2 }),
        // sra("sra t0,t1,t2", Sra { rd: T0, rs1: T1, rs2: T2 }),
        // or("or t0,t1,t2", Or { rd: T0, rs1: T1, rs2: T2 }),
        // and("and t0,t1,t2", And { rd: T0, rs1: T1, rs2: T2 }),
        // fence_tso("fence.tso", Fence { iorw: Iorw("tso".to_string()) }),
        // fence("fence io,rw", Fence { iorw: Iorw("io,rw".to_string()) }),
        // ecall("ecall", Ecall {}),
        // ebreak("ebreak", Ebreak {}),

        // // RV64I (12 tests)
        // lwu("lwu t0,-4(t1)", Lwu { rd: T0, imm: Immediate(-4), rs1: T1 }),
        // ld("ld t0,-4(t1)", Ld { rd: T0, imm: Immediate(-4), rs1: T1 }),
        // sd("sd t0,-4(t1)", Sd { rs2: T0, imm: Immediate(-4), rs1: T1 }),
        // addiw("addiw t0,t1,4", Addiw { rd: T0, rs1: T1, imm: Immediate(4) }),
        // slliw("slliw t0,t1,4", Slliw { rd: T0, rs1: T1, imm: Immediate(4) }),
        // srliw("srliw t0,t1,4", Srliw { rd: T0, rs1: T1, imm: Immediate(4) }),
        // sraiw("sraiw t0,t1,4", Sraiw { rd: T0, rs1: T1, imm: Immediate(4) }),
        // addw("addw t0,t1,t2", Addw { rd: T0, rs1: T1, rs2: T2 }),
        // subw("subw t0,t1,t2", Subw { rd: T0, rs1: T1, rs2: T2 }),
        // sllw("sllw t0,t1,t2", Sllw { rd: T0, rs1: T1, rs2: T2 }),
        // srlw("srlw t0,t1,t2", Srlw { rd: T0, rs1: T1, rs2: T2 }),
        // sraw("sraw t0,t1,t2", Sraw { rd: T0, rs1: T1, rs2: T2 }),

        // // RV32/RV64 Zifencei (1 test)
        // fence_i("fence.i", FenceI {}),

        // // RV32/RV64 Zicsr (10 tests)
        // csrrw("csrrw t0,cycle,t1", Csrrw { rd: T0, csr: Cycle, rs1: T1 }),
        // csrrs("csrrs t0,time,t1", Csrrs { rd: T0, csr: Time, rs1: T1 }),
        // csrrc("csrrc t0,instret,t1", Csrrc { rd: T0, csr: Instret, rs1: T1 }),
        // csrrwi("csrrwi t0,cycleh,4", Csrrwi { rd: T0, csr: Cycleh, imm: Immediate(4) }),
        // csrrsi("csrrsi t0,timeh,4", Csrrsi {rd: T0, csr: Timeh, imm: Immediate(4) }),
        // csrrci("csrrci t0,instreth,4", Csrrci {rd: T0, csr: Instreth, imm: Immediate(4) }),
        // csr_fflags("csrrc t0,fflags,t1", Csrrc { rd: T0, csr: Fflags, rs1: T1 }),
        // csr_frm("csrrc t0,frm,t1", Csrrc { rd: T0, csr: Frm, rs1: T1 }),
        // csr_fcsr("csrrc t0,fcsr,t1", Csrrc { rd: T0, csr: Fcsr, rs1: T1 }),
        // csr_unknown("csrrc t0,0x99,t1", Csrrc { rd: T0, csr: UnknownCsr("0x99".to_string()), rs1: T1 }),

        // // RV32M (8 tests)
        // mul("mul t0,t1,t2", Mul { rd: T0, rs1: T1, rs2: T2 }),
        // mulh("mulh t0,t1,t2", Mulh { rd: T0, rs1: T1, rs2: T2 }),
        // mulhsu("mulhsu t0,t1,t2", Mulhsu { rd: T0, rs1: T1, rs2: T2 }),
        // mulhu("mulhu t0,t1,t2", Mulhu { rd: T0, rs1: T1, rs2: T2 }),
        // div("div t0,t1,t2", Div { rd: T0, rs1: T1, rs2: T2 }),
        // divu("divu t0,t1,t2", Divu { rd: T0, rs1: T1, rs2: T2 }),
        // rem("rem t0,t1,t2", Rem { rd: T0, rs1: T1, rs2: T2 }),
        // remu("remu t0,t1,t2", Remu { rd: T0, rs1: T1, rs2: T2 }),

        // // RV64M (5 tests)
        // mulw("mulw t0,t1,t2", Mulw { rd: T0, rs1: T1, rs2: T2 }),
        // divw("divw t0,t1,t2", Divw { rd: T0, rs1: T1, rs2: T2 }),
        // divuw("divuw t0,t1,t2", Divuw { rd: T0, rs1: T1, rs2: T2 }),
        // remw("remw t0,t1,t2", Remw { rd: T0, rs1: T1, rs2: T2 }),
        // remuw("remuw t0,t1,t2", Remuw { rd: T0, rs1: T1, rs2: T2 }),

        // // RV32A (44 tests)
        // lr_w("lr.w t0,(t1)", LrW { ord: No, rd: T0, rs1: T1 }),
        // lr_w_aq("lr.w.aq t0,(t1)", LrW { ord: Aq, rd: T0, rs1: T1 }),
        // lr_w_rl("lr.w.rl t0,(t1)", LrW { ord: Rl, rd: T0, rs1: T1 }),
        // lr_w_aqrl("lr.w.aqrl t0,(t1)", LrW { ord: AqRl, rd: T0, rs1: T1 }),
        // sc_w("sc.w t0,t1,(t2)", ScW { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        // sc_w_aq("sc.w.aq t0,t1,(t2)", ScW { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        // sc_w_rl("sc.w.rl t0,t1,(t2)", ScW { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        // sc_w_aqrl("sc.w.aqrl t0,t1,(t2)", ScW { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        // amoswap_w("amoswap.w t0,t1,(t2)", AmoswapW { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        // amoswap_w_aq("amoswap.w.aq t0,t1,(t2)", AmoswapW { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        // amoswap_w_rl("amoswap.w.rl t0,t1,(t2)", AmoswapW { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        // amoswap_w_aqrl("amoswap.w.aqrl t0,t1,(t2)", AmoswapW { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        // amoadd_w("amoadd.w t0,t1,(t2)", AmoaddW { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        // amoadd_w_aq("amoadd.w.aq t0,t1,(t2)", AmoaddW { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        // amoadd_w_rl("amoadd.w.rl t0,t1,(t2)", AmoaddW { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        // amoadd_w_aqrl("amoadd.w.aqrl t0,t1,(t2)", AmoaddW { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        // amoxor_w("amoxor.w t0,t1,(t2)", AmoxorW { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        // amoxor_w_aq("amoxor.w.aq t0,t1,(t2)", AmoxorW { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        // amoxor_w_rl("amoxor.w.rl t0,t1,(t2)", AmoxorW { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        // amoxor_w_aqrl("amoxor.w.aqrl t0,t1,(t2)", AmoxorW { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        // amoand_w("amoand.w t0,t1,(t2)", AmoandW { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        // amoand_w_aq("amoand.w.aq t0,t1,(t2)", AmoandW { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        // amoand_w_rl("amoand.w.rl t0,t1,(t2)", AmoandW { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        // amoand_w_aqrl("amoand.w.aqrl t0,t1,(t2)", AmoandW { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        // amoor_w("amoor.w t0,t1,(t2)", AmoorW { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        // amoor_w_aq("amoor.w.aq t0,t1,(t2)", AmoorW { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        // amoor_w_rl("amoor.w.rl t0,t1,(t2)", AmoorW { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        // amoor_w_aqrl("amoor.w.aqrl t0,t1,(t2)", AmoorW { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        // amomin_w("amomin.w t0,t1,(t2)", AmominW { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        // amomin_w_aq("amomin.w.aq t0,t1,(t2)", AmominW { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        // amomin_w_rl("amomin.w.rl t0,t1,(t2)", AmominW { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        // amomin_w_aqrl("amomin.w.aqrl t0,t1,(t2)", AmominW { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        // amomax_w("amomax.w t0,t1,(t2)", AmomaxW { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        // amomax_w_aq("amomax.w.aq t0,t1,(t2)", AmomaxW { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        // amomax_w_rl("amomax.w.rl t0,t1,(t2)", AmomaxW { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        // amomax_w_aqrl("amomax.w.aqrl t0,t1,(t2)", AmomaxW { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        // amominu_w("amominu.w t0,t1,(t2)", AmominuW { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        // amominu_w_aq("amominu.w.aq t0,t1,(t2)", AmominuW { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        // amominu_w_rl("amominu.w.rl t0,t1,(t2)", AmominuW { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        // amominu_w_aqrl("amominu.w.aqrl t0,t1,(t2)", AmominuW { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        // amomaxu_w("amomaxu.w t0,t1,(t2)", AmomaxuW { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        // amomaxu_w_aq("amomaxu.w.aq t0,t1,(t2)", AmomaxuW { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        // amomaxu_w_rl("amomaxu.w.rl t0,t1,(t2)", AmomaxuW { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        // amomaxu_w_aqrl("amomaxu.w.aqrl t0,t1,(t2)", AmomaxuW { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),

        // // RV64A (44 tests)
        // lr_d("lr.d t0,(t1)", LrD { ord: No, rd: T0, rs1: T1 }),
        // lr_d_aq("lr.d.aq t0,(t1)", LrD { ord: Aq, rd: T0, rs1: T1 }),
        // lr_d_rl("lr.d.rl t0,(t1)", LrD { ord: Rl, rd: T0, rs1: T1 }),
        // lr_d_aqrl("lr.d.aqrl t0,(t1)", LrD { ord: AqRl, rd: T0, rs1: T1 }),
        // sc_d("sc.d t0,t1,(t2)", ScD { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        // sc_d_aq("sc.d.aq t0,t1,(t2)", ScD { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        // sc_d_rl("sc.d.rl t0,t1,(t2)", ScD { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        // sc_d_aqrl("sc.d.aqrl t0,t1,(t2)", ScD { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        // amoswap_d("amoswap.d t0,t1,(t2)", AmoswapD { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        // amoswap_d_aq("amoswap.d.aq t0,t1,(t2)", AmoswapD { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        // amoswap_d_rl("amoswap.d.rl t0,t1,(t2)", AmoswapD { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        // amoswap_d_aqrl("amoswap.d.aqrl t0,t1,(t2)", AmoswapD { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        // amoadd_d("amoadd.d t0,t1,(t2)", AmoaddD { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        // amoadd_d_aq("amoadd.d.aq t0,t1,(t2)", AmoaddD { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        // amoadd_d_rl("amoadd.d.rl t0,t1,(t2)", AmoaddD { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        // amoadd_d_aqrl("amoadd.d.aqrl t0,t1,(t2)", AmoaddD { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        // amoxor_d("amoxor.d t0,t1,(t2)", AmoxorD { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        // amoxor_d_aq("amoxor.d.aq t0,t1,(t2)", AmoxorD { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        // amoxor_d_rl("amoxor.d.rl t0,t1,(t2)", AmoxorD { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        // amoxor_d_aqrl("amoxor.d.aqrl t0,t1,(t2)", AmoxorD { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        // amoand_d("amoand.d t0,t1,(t2)", AmoandD { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        // amoand_d_aq("amoand.d.aq t0,t1,(t2)", AmoandD { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        // amoand_d_rl("amoand.d.rl t0,t1,(t2)", AmoandD { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        // amoand_d_aqrl("amoand.d.aqrl t0,t1,(t2)", AmoandD { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        // amoor_d("amoor.d t0,t1,(t2)", AmoorD { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        // amoor_d_aq("amoor.d.aq t0,t1,(t2)", AmoorD { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        // amoor_d_rl("amoor.d.rl t0,t1,(t2)", AmoorD { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        // amoor_d_aqrl("amoor.d.aqrl t0,t1,(t2)", AmoorD { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        // amomin_d("amomin.d t0,t1,(t2)", AmominD { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        // amomin_d_aq("amomin.d.aq t0,t1,(t2)", AmominD { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        // amomin_d_rl("amomin.d.rl t0,t1,(t2)", AmominD { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        // amomin_d_aqrl("amomin.d.aqrl t0,t1,(t2)", AmominD { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        // amomax_d("amomax.d t0,t1,(t2)", AmomaxD { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        // amomax_d_aq("amomax.d.aq t0,t1,(t2)", AmomaxD { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        // amomax_d_rl("amomax.d.rl t0,t1,(t2)", AmomaxD { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        // amomax_d_aqrl("amomax.d.aqrl t0,t1,(t2)", AmomaxD { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        // amominu_d("amominu.d t0,t1,(t2)", AmominuD { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        // amominu_d_aq("amominu.d.aq t0,t1,(t2)", AmominuD { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        // amominu_d_rl("amominu.d.rl t0,t1,(t2)", AmominuD { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        // amominu_d_aqrl("amominu.d.aqrl t0,t1,(t2)", AmominuD { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        // amomaxu_d("amomaxu.d t0,t1,(t2)", AmomaxuD { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        // amomaxu_d_aq("amomaxu.d.aq t0,t1,(t2)", AmomaxuD { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        // amomaxu_d_rl("amomaxu.d.rl t0,t1,(t2)", AmomaxuD { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        // amomaxu_d_aqrl("amomaxu.d.aqrl t0,t1,(t2)", AmomaxuD { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),

        // // RV32F (33 tests)
        // rm_rne("fmadd.s ft0,ft1,ft2,ft3,rne", FmaddS { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Rne }),
        // rm_rtz("fmadd.s ft0,ft1,ft2,ft3,rtz", FmaddS { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Rtz }),
        // rm_rdn("fmadd.s ft0,ft1,ft2,ft3,rdn", FmaddS { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Rdn }),
        // rm_rup("fmadd.s ft0,ft1,ft2,ft3,rup", FmaddS { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Rup }),
        // rm_rmm("fmadd.s ft0,ft1,ft2,ft3,rmm", FmaddS { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Rmm }),
        // rm_dyn("fmadd.s ft0,ft1,ft2,ft3,dyn", FmaddS { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Dyn }),
        // rm_default("fmadd.s ft0,ft1,ft2,ft3", FmaddS { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Dyn }),

        // flw("flw ft0,-4(t0)", Flw { frd: Ft0, imm: Immediate(-4), rs1: T0 }),
        // fsw("fsw ft0,-4(t0)", Fsw { frs2: Ft0, imm: Immediate(-4), rs1: T0 }),
        // fmadd_s("fmadd.s ft0,ft1,ft2,ft3", FmaddS { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Dyn }),
        // fmsub_s("fmsub.s ft0,ft1,ft2,ft3", FmsubS { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Dyn }),
        // fnmsub_s("fnmsub.s ft0,ft1,ft2,ft3", FnmsubS { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Dyn }),
        // fnmadd_s("fnmadd.s ft0,ft1,ft2,ft3", FnmaddS { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Dyn }),
        // fadd_s("fadd.s ft0,ft1,ft2", FaddS { frd: Ft0, frs1: Ft1, frs2: Ft2, rm: Dyn }),
        // fsub_s("fsub.s ft0,ft1,ft2", FsubS { frd: Ft0, frs1: Ft1, frs2: Ft2, rm: Dyn }),
        // fmul_s("fmul.s ft0,ft1,ft2", FmulS { frd: Ft0, frs1: Ft1, frs2: Ft2, rm: Dyn }),
        // fdiv_s("fdiv.s ft0,ft1,ft2", FdivS { frd: Ft0, frs1: Ft1, frs2: Ft2, rm: Dyn }),
        // fsqrt_s("fsqrt.s ft0,ft1", FsqrtS { frd: Ft0, frs1: Ft1, rm: Dyn }),
        // fsgnj_s("fsgnj.s ft0,ft1,ft2", FsgnjS { frd: Ft0, frs1: Ft1, frs2: Ft2 }),
        // fsgnjn_s("fsgnjn.s ft0,ft1,ft2", FsgnjnS { frd: Ft0, frs1: Ft1, frs2: Ft2 }),
        // fsgnjx_s("fsgnjx.s ft0,ft1,ft2", FsgnjxS { frd: Ft0, frs1: Ft1, frs2: Ft2 }),
        // fmin_s("fmin.s ft0,ft1,ft2", FminS { frd: Ft0, frs1: Ft1, frs2: Ft2 }),
        // fmax_s("fmax.s ft0,ft1,ft2", FmaxS { frd: Ft0, frs1: Ft1, frs2: Ft2 }),
        // fcvt_w_s("fcvt.w.s t0,ft0", FcvtWS { rd: T0, frs1: Ft0, rm: Dyn }),
        // fcvt_wu_s("fcvt.wu.s t0,ft0", FcvtWuS { rd: T0, frs1: Ft0, rm: Dyn }),
        // fmv_x_w("fmv.x.w t0,ft0", FmvXW { rd: T0, frs1: Ft0 }),
        // feq_s("feq.s t0,ft0,ft1", FeqS { rd: T0, frs1: Ft0, frs2: Ft1 }),
        // flt_s("flt.s t0,ft0,ft1", FltS { rd: T0, frs1: Ft0, frs2: Ft1 }),
        // fle_s("fle.s t0,ft0,ft1", FleS { rd: T0, frs1: Ft0, frs2: Ft1 }),
        // fclass_s("fclass.s t0,ft0", FclassS { rd: T0, frs1: Ft0 }),
        // fcvt_s_w("fcvt.s.w ft0,t0", FcvtSW { frd: Ft0, rs1: T0, rm: Dyn }),
        // fcvt_s_wu("fcvt.s.wu ft0,t0", FcvtSWu { frd: Ft0, rs1: T0, rm: Dyn }),
        // fmv_w_x("fmv.w.x ft0,t0", FmvWX { frd: Ft0, rs1: T0 }),

        // // RV64F (4 tests)
        // fcvt_l_s("fcvt.l.s t0,ft0", FcvtLS { rd: T0, frs1: Ft0, rm: Dyn }),
        // fcvt_lu_s("fcvt.lu.s t0,ft0", FcvtLuS { rd: T0, frs1: Ft0, rm: Dyn }),
        // fcvt_s_l("fcvt.s.l ft0,t0", FcvtSL { frd: Ft0, rs1: T0, rm: Dyn }),
        // fcvt_s_lu("fcvt.s.lu ft0,t0", FcvtSLu { frd: Ft0, rs1: T0, rm: Dyn }),

        // // RV32D (26 tests)
        // fld("fld ft0,-4(t0)", Fld { frd: Ft0, imm: Immediate(-4), rs1: T0 }),
        // fsd("fsd ft0,-4(t0)", Fsd { frs2: Ft0, imm: Immediate(-4), rs1: T0 }),
        // fmadd_d("fmadd.d ft0,ft1,ft2,ft3", FmaddD { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Dyn }),
        // fmsub_d("fmsub.d ft0,ft1,ft2,ft3", FmsubD { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Dyn }),
        // fnmsub_d("fnmsub.d ft0,ft1,ft2,ft3", FnmsubD { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Dyn }),
        // fnmadd_d("fnmadd.d ft0,ft1,ft2,ft3", FnmaddD { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Dyn }),
        // fadd_d("fadd.d ft0,ft1,ft2", FaddD { frd: Ft0, frs1: Ft1, frs2: Ft2, rm: Dyn }),
        // fsub_d("fsub.d ft0,ft1,ft2", FsubD { frd: Ft0, frs1: Ft1, frs2: Ft2, rm: Dyn }),
        // fmul_d("fmul.d ft0,ft1,ft2", FmulD { frd: Ft0, frs1: Ft1, frs2: Ft2, rm: Dyn }),
        // fdiv_d("fdiv.d ft0,ft1,ft2", FdivD { frd: Ft0, frs1: Ft1, frs2: Ft2, rm: Dyn }),
        // fsqrt_d("fsqrt.d ft0,ft1", FsqrtD { frd: Ft0, frs1: Ft1, rm: Dyn }),
        // fsgnj_d("fsgnj.d ft0,ft1,ft2", FsgnjD { frd: Ft0, frs1: Ft1, frs2: Ft2 }),
        // fsgnjn_d("fsgnjn.d ft0,ft1,ft2", FsgnjnD { frd: Ft0, frs1: Ft1, frs2: Ft2 }),
        // fsgnjx_d("fsgnjx.d ft0,ft1,ft2", FsgnjxD { frd: Ft0, frs1: Ft1, frs2: Ft2 }),
        // fmin_d("fmin.d ft0,ft1,ft2", FminD { frd: Ft0, frs1: Ft1, frs2: Ft2 }),
        // fmax_d("fmax.d ft0,ft1,ft2", FmaxD { frd: Ft0, frs1: Ft1, frs2: Ft2 }),
        // fcvt_s_d("fcvt.s.d ft0,ft1", FcvtSD { frd: Ft0, frs1: Ft1, rm: Dyn }),
        // fcvt_d_s("fcvt.d.s ft0,ft1", FcvtDS { frd: Ft0, frs1: Ft1, rm: Dyn }),
        // feq_d("feq.d t0,ft0,ft1", FeqD { rd: T0, frs1: Ft0, frs2: Ft1 }),
        // flt_d("flt.d t0,ft0,ft1", FltD { rd: T0, frs1: Ft0, frs2: Ft1 }),
        // fle_d("fle.d t0,ft0,ft1", FleD { rd: T0, frs1: Ft0, frs2: Ft1 }),
        // fclass_d("fclass.d t0,ft0", FclassD { rd: T0, frs1: Ft0 }),
        // fcvt_w_d("fcvt.w.d t0,ft0", FcvtWD { rd: T0, frs1: Ft0, rm: Dyn }),
        // fcvt_wu_d("fcvt.wu.d t0,ft0", FcvtWuD { rd: T0, frs1: Ft0, rm: Dyn }),
        // fcvt_d_w("fcvt.d.w ft0,t0", FcvtDW { frd: Ft0, rs1: T0, rm: Dyn }),
        // fcvt_d_wu("fcvt.d.wu ft0,t0", FcvtDWu { frd: Ft0, rs1: T0, rm: Dyn }),

        // // RV64D (6 tests)
        // fcvt_l_d("fcvt.l.d t0,ft0", FcvtLD { rd: T0, frs1: Ft0, rm: Dyn }),
        // fcvt_lu_d("fcvt.lu.d t0,ft0", FcvtLuD { rd: T0, frs1: Ft0, rm: Dyn }),
        // fmv_x_d("fmv.x.d t0,ft0", FmvXD { rd: T0, frs1: Ft0 }),
        // fcvt_d_l("fcvt.d.l ft0,t0", FcvtDL { frd: Ft0, rs1: T0, rm: Dyn }),
        // fcvt_d_lu("fcvt.d.lu ft0,t0", FcvtDLu { frd: Ft0, rs1: T0, rm: Dyn }),
        // fmv_d_x("fmv.d.x ft0,t0", FmvDX { frd: Ft0, rs1: T0 }),

        // // Pseudoinstructions (50 tests)
        // nop("nop", Nop {}),
        // li("li t0,4", Li { rd: T0, imm: Immediate(4) }),
        // mv("mv t0,t1", Mv { rd: T0, rs1: T1 }),
        // not("not t0,t1", Not { rd: T0, rs1: T1 }),
        // neg("neg t0,t1", Neg { rd: T0, rs1: T1 }),
        // negw("negw t0,t1", Negw { rd: T0, rs1: T1 }),
        // sext_w("sext.w t0,t1", SextW { rd: T0, rs1: T1 }),
        // seqz("seqz t0,t1", Seqz { rd: T0, rs1: T1 }),
        // snez("snez t0,t1", Snez { rd: T0, rs1: T1 }),
        // sltz("sltz t0,t1", Sltz { rd: T0, rs1: T1 }),
        // sgtz("sgtz t0,t1", Sgtz { rd: T0, rs1: T1 }),

        // fmv_s("fmv.s ft0,ft1", FmvS { frd: Ft0, frs1: Ft1 }),
        // fabs_s("fabs.s ft0,ft1", FabsS { frd: Ft0, frs1: Ft1 }),
        // fneg_s("fneg.s ft0,ft1", FnegS { frd: Ft0, frs1: Ft1 }),
        // fmv_d("fmv.d ft0,ft1", FmvD { frd: Ft0, frs1: Ft1 }),
        // fabs_d("fabs.d ft0,ft1", FabsD { frd: Ft0, frs1: Ft1 }),
        // fneg_d("fneg.d ft0,ft1", FnegD { frd: Ft0, frs1: Ft1 }),

        // beqz("beqz t0,main", Beqz { rs1: T0, addr: Address(0x0) }),
        // bnez("bnez t0,main", Bnez { rs1: T0, addr: Address(0x0) }),
        // blez("blez t0,main", Blez { rs1: T0, addr: Address(0x0) }),
        // bgez("bgez t0,main", Bgez { rs1: T0, addr: Address(0x0) }),
        // bltz("bltz t0,main", Bltz { rs1: T0, addr: Address(0x0) }),
        // bgtz("bgtz t0,main", Bgtz { rs1: T0, addr: Address(0x0) }),

        // j("j main", J { addr: Address(0x0) }),
        // jr("jr t0", Jr { rs1: T0 }),
        // pseudo_jalr("jalr t0", PseudoJalr { rs1: T0 }),
        // ret("ret", Ret {}),

        // pseudo_fence("fence", PseudoFence {}),

        // rdinstret("rdinstret t0", Rdinstret { rd: T0 }),
        // rdinstreth("rdinstreth t0", Rdinstreth { rd: T0 }),
        // rdcycle("rdcycle t0", Rdcycle { rd: T0 }),
        // rdcycleh("rdcycleh t0", Rdcycleh { rd: T0 }),
        // rdtime("rdtime t0", Rdtime { rd: T0 }),
        // rdtimeh("rdtimeh t0", Rdtimeh { rd: T0 }),

        // csrr("csrr t0, 0x99", Csrr { rd: T0, csr: UnknownCsr("0x99".to_string()) }),
        // csrw("csrw 0x99, t0", Csrw { csr: UnknownCsr("0x99".to_string()), rs1: T0 }),
        // csrs("csrs 0x99, t0", Csrs { csr: UnknownCsr("0x99".to_string()), rs1: T0 }),
        // csrc("csrc 0x99, t0", Csrc { csr: UnknownCsr("0x99".to_string()), rs1: T0 }),

        // csrwi("csrwi 0x99, 4", Csrwi { csr: UnknownCsr("0x99".to_string()), imm: Immediate(4) }),
        // csrsi("csrsi 0x99, 4", Csrsi { csr: UnknownCsr("0x99".to_string()), imm: Immediate(4) }),
        // csrci("csrci 0x99, 4", Csrci { csr: UnknownCsr("0x99".to_string()), imm: Immediate(4) }),

        // frcsr("frcsr t0", Frcsr { rd: T0 }),
        // fscsr("fscsr t0,t1", Fscsr { rd: T0, rs1: T1 }),
        // fwcsr("fscsr t0", Fwcsr { rs1: T0 }),

        // frrm("frrm t0", Frrm { rd: T0 }),
        // fsrm("fsrm t0,t1", Fsrm { rd: T0, rs1: T1 }),
        // fwrm("fsrm t0", Fwrm { rs1: T0 }),

        // frflags("frflags t0", Frflags { rd: T0 }),
        // fsflags("fsflags t0,t1", Fsflags { rd: T0, rs1: T1 }),
        // fwflags("fsflags t0", Fwflags { rs1: T0 }),
    }
}
