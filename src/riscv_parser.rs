use crate::riscv_isa::{Abi, Address, CodeBlock, DataBlock, Instruction, Program};
use regex::Regex;
use std::mem;

lazy_static! {
    static ref RODATA_SECTION: Regex = Regex::new(r"Disassembly of section \.rodata:").unwrap();
    static ref CODE_SECTION: Regex = Regex::new(r"Disassembly of section (\.text.*):").unwrap();
    static ref NOT_DATA_SECTION: Regex =
        Regex::new(r"Disassembly of section (\.comment)|(\.debug.*):").unwrap();
    static ref SECTION: Regex = Regex::new(r"Disassembly of section (.+):").unwrap();
    static ref SYMBOL: Regex = Regex::new(r"([[:xdigit:]]+) <(.*)>:").unwrap();
    static ref BYTES: Regex = Regex::new(r"[[:xdigit:]]+:\s+([[:xdigit:]]+)").unwrap();
}

pub struct Parser<'a> {
    lines: Vec<&'a str>,
    jump_table: Vec<Address>,
    abi: Abi,
    code_blocks: Vec<CodeBlock>,
    data_blocks: Vec<DataBlock>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, abi: &Option<String>) -> Self {
        let lines: Vec<_> = source
            .lines()
            .map(|l| l.trim())
            .filter(|l| l.is_empty())
            .skip_while(|l| !SECTION.is_match(l))
            .collect();
        assert!(!lines.is_empty(), "Empty disassembly");
        Parser {
            lines,
            jump_table: Vec::new(),
            abi: Abi::new(abi),
            code_blocks: Vec::new(),
            data_blocks: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Program {
        let rodata: Vec<_> = self
            .lines
            .iter()
            .cloned()
            .skip_while(|l| RODATA_SECTION.is_match(l))
            .take_while(|l| !SECTION.is_match(l))
            .collect();
        let data_blocks = self.parse_rodata_section(&rodata);
        self.data_blocks.extend(data_blocks);

        let mut start = 0;
        let mut end = self.lines[1..]
            .iter()
            .position(|l| SECTION.is_match(l))
            .unwrap_or(self.lines.len());
        while end <= self.lines.len() {
            let section = &self.lines[start..end];
            if CODE_SECTION.is_match(section[0]) {
                let code_blocks = self.parse_code_section(section);
                self.code_blocks.extend(code_blocks);
            } else if !NOT_DATA_SECTION.is_match(section[0]) {
                let data_blocks = self.parse_data_section(section);
                self.data_blocks.extend(data_blocks);
            }

            if end == self.lines.len() {
                break;
            } else {
                start = end;
                end = self.lines[end + 1..]
                    .iter()
                    .position(|l| SECTION.is_match(l))
                    .unwrap_or(self.lines.len());
            }
        }

        Program {
            abi: mem::take(&mut self.abi),
            code_blocks: mem::take(&mut self.code_blocks),
            data_blocks: mem::take(&mut self.data_blocks),
        }
    }

    fn parse_rodata_section(&mut self, lines: &[&str]) -> Vec<DataBlock> {
        let data_blocks = self.parse_data_section(lines);
        let step = match self.abi {
            Abi::Ilp32 | Abi::Ilp32f | Abi::Ilp32d => 4,
            Abi::Lp64 | Abi::Lp64f | Abi::Lp64d => 8,
        };
        for data_block in data_blocks.iter() {
            assert!(
                data_block.bytes.len() % step == 0,
                "`.rodata` contains something other than the jump table"
            );
            let mut i = 0;
            while i < data_block.bytes.len() {
                let bytes = &data_block.bytes[i..i + step];
                let address = Address(usize::from_be_bytes(bytes.try_into().unwrap()));
                self.jump_table.push(address);
                i += step;
            }
        }
        data_blocks
    }

    fn parse_data_section(&self, lines: &[&str]) -> Vec<DataBlock> {
        let mut data_blocks = Vec::new();
        let section = SECTION.captures(lines[0]).unwrap()[1].to_string();
        let caps = SYMBOL.captures(lines[1]).unwrap();
        let mut symbol = caps[1].to_string();
        let mut address = Address::new(&caps[0]);
        let mut bytes = Vec::new();

        for line in &lines[2..] {
            if let Some(caps) = SYMBOL.captures(line) {
                let symbol = mem::replace(&mut symbol, caps[1].to_string());
                let address = mem::replace(&mut address, Address::new(&caps[0]));
                let bytes = mem::take(&mut bytes);
                data_blocks.push(DataBlock {
                    section: section.clone(),
                    symbol,
                    address,
                    bytes,
                });
            } else {
                let bytes_str = &BYTES.captures(line).unwrap()[1];
                bytes.extend(match bytes_str.len() {
                    4 => u16::from_str_radix(bytes_str, 16)
                        .unwrap()
                        .to_be_bytes()
                        .to_vec(),
                    8 => u32::from_str_radix(bytes_str, 16)
                        .unwrap()
                        .to_be_bytes()
                        .to_vec(),
                    _ => unreachable!(),
                });
            }
        }

        data_blocks.push(DataBlock {
            section,
            symbol,
            address,
            bytes,
        });

        data_blocks
    }

    fn parse_code_section(&self, lines: &[&str]) -> Vec<CodeBlock> {
        use Instruction::*;

        let mut code_blocks = Vec::new();
        let section = SECTION.captures(lines[0]).unwrap()[1].to_string();
        let caps = SYMBOL.captures(lines[1]).unwrap();
        let mut symbol = caps[1].to_string();
        let mut address = Address::new(&caps[0]);
        let mut instructions = Vec::new();
        let mut split = false;

        for line in &lines[2..] {
            if let Some(caps) = SYMBOL.captures(line) {
                let symbol = mem::replace(&mut symbol, caps[1].to_string());
                let address = mem::replace(&mut address, Address::new(&caps[0]));
                let instructions = mem::take(&mut instructions);
                code_blocks.push(CodeBlock {
                    section: section.clone(),
                    symbol,
                    address,
                    instructions,
                });
            } else {
                let inst = Instruction::new(line);
                let addr = inst.address();

                if split || self.jump_table.iter().any(|a| a == addr) {
                    split = false;
                    let Address(addr) = addr;
                    let symbol = mem::replace(&mut symbol, addr.to_string());
                    let address = mem::replace(&mut address, Address(*addr));
                    let instructions = mem::take(&mut instructions);
                    code_blocks.push(CodeBlock {
                        section: section.clone(),
                        symbol,
                        address,
                        instructions,
                    });
                }

                if let Jal { .. }
                | Jalr { .. }
                | Beq { .. }
                | Bne { .. }
                | Blt { .. }
                | Bge { .. }
                | Bltu { .. }
                | Bgeu { .. }
                | Beqz { .. }
                | Bnez { .. }
                | Blez { .. }
                | Bgez { .. }
                | Bltz { .. }
                | Bgtz { .. }
                | J { .. }
                | Jr { .. }
                | PseudoJalr { .. }
                | Ret { .. } = inst
                {
                    split = true;
                }

                instructions.push(inst);
            }
        }

        code_blocks.push(CodeBlock {
            section,
            symbol,
            address,
            instructions,
        });

        code_blocks
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::build_test;
    use crate::riscv_isa::{
        Abi, Address, Immediate,
        Instruction::{self, *},
        Program, Register,
    };
    use std::collections::HashMap;
    use std::env;
    use std::io::Write;
    use std::process::{Command, Stdio};
    use tempfile::NamedTempFile;

    fn compile_and_dump(source: &str, args: &[&str]) -> String {
        let temp_file = NamedTempFile::new().expect("Unable to create temp files");
        let gcc_var = env::var("gcc").expect("`$gcc` is not set");
        let objdump_var = env::var("objdump").expect("`$objdump` is not set");

        let mut gcc_proc = Command::new(gcc_var)
            .args(args)
            .args([
                "-c",
                "-x",
                "assembler",
                "-",
                "-o",
                temp_file.path().to_str().unwrap(),
            ])
            .stdin(Stdio::piped())
            .spawn()
            .expect("Unable to invoke `$gcc`");
        gcc_proc
            .stdin
            .as_mut()
            .unwrap()
            .write_all(source.as_bytes())
            .unwrap();
        gcc_proc.wait().unwrap();

        let objdump_proc = Command::new(objdump_var)
            .args([
                "-D",
                "-j.text",
                "-j.rodata",
                "-j.data",
                "-j.bss",
                "-j.sdata",
                "-j.sbss",
                "-wz",
                temp_file.path().to_str().unwrap(),
            ])
            .output()
            .expect("Unable to invoke `$objdump`");
        String::from_utf8(objdump_proc.stdout).unwrap()
    }

    // #[test]
    // fn lui() {
    //     let source = "
    //         main:
    //             lui zero,0
    //             ret
    //     ";
    //     let source = compile_and_dump(source);
    //     let program = Parser::new(&source).run();
    //     assert_eq!(
    //         program,
    //         Program {
    //             functions: vec![Function {
    //                 name: String::from("main"),
    //                 basic_blocks: vec![BasicBlock {
    //                     instructions: vec![
    //                         Instruction::Lui {
    //                             label: Some(String::from("main")),
    //                             address: Address(0x0),
    //                             rd: Register::Zero,
    //                             imm: Immediate(0),
    //                             comment: None,
    //                         },
    //                         Instruction::Ret {
    //                             label: None,
    //                             address: Address(0x4),
    //                             comment: None,
    //                         }
    //                     ],
    //                     continue_target: None,
    //                     jump_target: None,
    //                 }],
    //                 indirect_targets: HashMap::new(),
    //             }],
    //             data: HashMap::new(),
    //         }
    //     );
    // }

    // #[test]
    // fn basic() {
    //     let source = "";

    //     let indirect_targets = super::parse_rodata("");
    //     assert!(indirect_targets.is_empty());

    //     let indirect_targets = super::parse_rodata(source);
    //     let mut expected: HashMap<RiscvAddress, RiscvAddress> = HashMap::new();
    //     expected.insert(0x10594.into(), 0x000104ba.into());
    //     expected.insert(0x10598.into(), 0x0001047e.into());
    //     assert_eq!(indirect_targets, expected);

    //     let statics = super::parse_sdata(source);
    //     let mut expected = HashMap::new();
    //     expected.insert(
    //         "_IO_stdin_used".to_string(),
    //         ("0000000000020001".to_string(), LlvmType::I8),
    //     );
    //     expected.insert("__dso_handle".to_string(), ("".to_string(), LlvmType::I8));
    //     expected.insert("g1".to_string(), ("00000001".to_string(), LlvmType::I8));
    //     assert_eq!(statics, expected);

    //     let statics = super::parse_sbss(source);
    //     let mut expected = HashMap::new();
    //     expected.insert("g2".to_string(), ("".to_string(), LlvmType::I8));
    //     assert_eq!(statics, expected);

    //     let insts = super::parse_text(source);
    //     let expected = vec![
    //         Lui {
    //             label: Some(String::from("main")),
    //             address: 0x103ea.into(),
    //             rd: A0,
    //             imm: RiscvImmediate::new("0x12"),
    //             comment: None,
    //         },
    //         Lui {
    //             label: None,
    //             address: 0x103ea.into(),
    //             rd: A0,
    //             imm: RiscvImmediate::new("0x12"),
    //             comment: Some(String::from("<deregister_tm_clones+0x1c>")),
    //         },
    //     ];
    //     assert_eq!(insts, expected);
    // }

    build_test! {
        // Registers (32 tests)
        // reg_1("flw	ft0,-20(zero)", Flw { rd: Ft0, imm: (-20).into(), rs1: Zero }),
    //     reg_2("flw	ft1,-20(ra)", Flw { rd: Ft1, imm: (-20).into(), rs1: Ra }),
    //     reg_3("flw	ft2,-20(sp)", Flw { rd: Ft2, imm: (-20).into(), rs1: Sp }),
    //     reg_4("flw	ft3,-20(gp)", Flw { rd: Ft3, imm: (-20).into(), rs1: Gp }),
    //     reg_5("flw	ft4,-20(tp)", Flw { rd: Ft4, imm: (-20).into(), rs1: Tp }),
    //     reg_6("flw	ft5,-20(t0)", Flw { rd: Ft5, imm: (-20).into(), rs1: T0 }),
    //     reg_7("flw	ft6,-20(t1)", Flw { rd: Ft6, imm: (-20).into(), rs1: T1 }),
    //     reg_8("flw	ft7,-20(t2)", Flw { rd: Ft7, imm: (-20).into(), rs1: T2 }),
    //     reg_9("flw	fs0,-20(s0)", Flw { rd: Fs0, imm: (-20).into(), rs1: S0 }),
    //     reg_10("flw	fs1,-20(s1)", Flw { rd: Fs1, imm: (-20).into(), rs1: S1 }),
    //     reg_11("flw	fa0,-20(a0)", Flw { rd: Fa0, imm: (-20).into(), rs1: A0 }),
    //     reg_12("flw	fa1,-20(a1)", Flw { rd: Fa1, imm: (-20).into(), rs1: A1 }),
    //     reg_13("flw	fa2,-20(a2)", Flw { rd: Fa2, imm: (-20).into(), rs1: A2 }),
    //     reg_14("flw	fa3,-20(a3)", Flw { rd: Fa3, imm: (-20).into(), rs1: A3 }),
    //     reg_15("flw	fa4,-20(a4)", Flw { rd: Fa4, imm: (-20).into(), rs1: A4 }),
    //     reg_16("flw	fa5,-20(a5)", Flw { rd: Fa5, imm: (-20).into(), rs1: A5 }),
    //     reg_17("flw	fa6,-20(a6)", Flw { rd: Fa6, imm: (-20).into(), rs1: A6 }),
    //     reg_18("flw	fa7,-20(a7)", Flw { rd: Fa7, imm: (-20).into(), rs1: A7 }),
    //     reg_19("flw	fs2,-20(s2)", Flw { rd: Fs2, imm: (-20).into(), rs1: S2 }),
    //     reg_20("flw	fs3,-20(s3)", Flw { rd: Fs3, imm: (-20).into(), rs1: S3 }),
    //     reg_21("flw	fs4,-20(s4)", Flw { rd: Fs4, imm: (-20).into(), rs1: S4 }),
    //     reg_22("flw	fs5,-20(s5)", Flw { rd: Fs5, imm: (-20).into(), rs1: S5 }),
    //     reg_23("flw	fs6,-20(s6)", Flw { rd: Fs6, imm: (-20).into(), rs1: S6 }),
    //     reg_24("flw	fs7,-20(s7)", Flw { rd: Fs7, imm: (-20).into(), rs1: S7 }),
    //     reg_25("flw	fs8,-20(s8)", Flw { rd: Fs8, imm: (-20).into(), rs1: S8 }),
    //     reg_26("flw	fs9,-20(s9)", Flw { rd: Fs9, imm: (-20).into(), rs1: S9 }),
    //     reg_27("flw	fs10,-20(s10)", Flw { rd: Fs10, imm: (-20).into(), rs1: S10 }),
    //     reg_28("flw	fs11,-20(s11)", Flw { rd: Fs11, imm: (-20).into(), rs1: S11 }),
    //     reg_29("flw	ft8,-20(t3)", Flw { rd: Ft8, imm: (-20).into(), rs1: T3 }),
    //     reg_30("flw	ft9,-20(t4)", Flw { rd: Ft9, imm: (-20).into(), rs1: T4 }),
    //     reg_31("flw	ft10,-20(t5)", Flw { rd: Ft10, imm: (-20).into(), rs1: T5 }),
    //     reg_32("flw	ft11,-20(t6)", Flw { rd: Ft11, imm: (-20).into(), rs1: T6 }),

    //     // RV32I (45 tests)
        lui("lui	zero,0", Lui { rd: Register::Zero, imm: Immediate(0) }, ["1", "2"]),
    //     auipc("auipc	a0,0x0", Auipc { rd: A0, imm: 0x0.into() }),
    //     jal("jal	ra,103de", Jal { rd: Ra, addr: 0x103de.into() }),
    //     jalr("jalr	t1,1(t0)", Jalr { rd: T1, imm: 1.into(), rs1: T0 }),
    //     jalr_imm_rs1("jalr	1(t0)", Jalr { rd: Ra, imm: 1.into(), rs1: T0 }),
    //     jalr_rd_rs1("jalr	t1,t0", Jalr { rd: T1, imm: 0.into(), rs1: T0 }),
    //     jalr_rs1("jalr	t0", Jalr { rd: Ra, imm: 0.into(), rs1: T0 }),
    //     beq("beq	a4,a5,10406", Beq { rs1: A4, rs2: A5, addr: 0x10406.into() }),
    //     bne("bne	a4,a5,10406", Bne { rs1: A4, rs2: A5, addr: 0x10406.into() }),
    //     blt("blt	a4,a5,10406", Blt { rs1: A4, rs2: A5, addr: 0x10406.into() }),
    //     bge("bge	a4,a5,10406", Bge { rs1: A4, rs2: A5, addr: 0x10406.into() }),
    //     bltu("bltu	a4,a5,10406", Bltu { rs1: A4, rs2: A5, addr: 0x10406.into() }),
    //     bgeu("bgeu	a4,a5,10406", Bgeu { rs1: A4, rs2: A5, addr: 0x10406.into() }),
    //     lb("lb	a5,-20(s0)", Lb { rd: A5, imm: (-20).into(), rs1: S0 }),
    //     lh("lh	a5,-20(s0)", Lh { rd: A5, imm: (-20).into(), rs1: S0 }),
    //     lw("lw	a5,-20(s0)", Lw { rd: A5, imm: (-20).into(), rs1: S0 }),
    //     lbu("lbu	a5,-20(s0)", Lbu { rd: A5, imm: (-20).into(), rs1: S0 }),
    //     lhu("lhu	a5,-20(s0)", Lhu { rd: A5, imm: (-20).into(), rs1: S0 }),
    //     sb("sb	a5,-2000(gp)", Sb { rs2: A5, imm: (-2000).into(), rs1: Gp }),
    //     sh("sh	a5,-2000(gp)", Sh { rs2: A5, imm: (-2000).into(), rs1: Gp }),
    //     sw("sw	a5,-2000(gp)", Sw { rs2: A5, imm: (-2000).into(), rs1: Gp }),
    //     addi("addi	a2,sp,8", Addi { rd: A2, rs1: Sp, imm: 8.into() }),
    //     slti("slti	t0,t1,0", Slti { rd: T0, rs1: T1, imm: 0.into() }),
    //     sltiu("sltiu	t0,t1,0", Sltiu { rd: T0, rs1: T1, imm: 0.into() }),
    //     xori("xori	t0,t1,0", Xori { rd: T0, rs1: T1, imm: 0.into() }),
    //     ori("ori	t0,t1,0", Ori { rd: T0, rs1: T1, imm: 0.into() }),
    //     andi("andi	t0,t1,0", Andi { rd: T0, rs1: T1, imm: 0.into() }),
    //     slli("slli	a4,a5,0x2", Slli { rd: A4, rs1: A5, imm: 0x2.into() }),
    //     srli("srli	a5,a1,0x3f", Srli { rd: A5, rs1: A1, imm: 0x3f.into() }),
    //     srai("srai	a5,a1,0x3", Srai { rd: A5, rs1: A1, imm: 0x3.into() }),
    //     add("add	t0,t1,t2", Add { rd: T0, rs1: T1, rs2: T2 }),
    //     sub("sub	t0,t1,t2", Sub { rd: T0, rs1: T1, rs2: T2 }),
    //     sll("sll	t0,t1,t2", Sll { rd: T0, rs1: T1, rs2: T2 }),
    //     slt("slt	t0,t1,t2", Slt { rd: T0, rs1: T1, rs2: T2 }),
    //     sltu("sltu	t0,t1,t2", Sltu { rd: T0, rs1: T1, rs2: T2 }),
    //     xor("xor	t0,t1,t2", Xor { rd: T0, rs1: T1, rs2: T2 }),
    //     srl("srl	t0,t1,t2", Srl { rd: T0, rs1: T1, rs2: T2 }),
    //     sra("sra	t0,t1,t2", Sra { rd: T0, rs1: T1, rs2: T2 }),
    //     or("or	t0,t1,t2", Or { rd: T0, rs1: T1, rs2: T2 }),
    //     and("and	t0,t1,t2", And { rd: T0, rs1: T1, rs2: T2 }),
    //     fence("fence", Fence {}),
    //     fence_iorw("fence	io,rw", Fence {}),
    //     fence_tso("fence.tso", Fence {}),
    //     ecall("ecall", Ecall {}),
    //     ebreak("ebreak", Ebreak {}),

    //     // RV64I (12 tests)
    //     lwu("lwu	a5,-20(s0)", Lwu { rd: A5, imm: (-20).into(), rs1: S0 }),
    //     ld("ld	a1,0(sp)", Ld { rd: A1, imm: 0.into(), rs1: Sp }),
    //     sd("sd	s0,0(sp)", Sd { rs2: S0, imm: 0.into(), rs1: Sp }),
    //     addiw("addiw	t0,t1,1", Addiw { rd: T0, rs1: T1, imm: 1.into() }),
    //     slliw("slliw	a4,a5,0x2", Slliw { rd: A4, rs1: A5, imm: 0x2.into() }),
    //     srliw("srliw	a4,a5,0x2", Srliw { rd: A4, rs1: A5, imm: 0x2.into() }),
    //     sraiw("sraiw	a4,a5,0x2", Sraiw { rd: A4, rs1: A5, imm: 0x2.into() }),
    //     addw("addw	t0,t1,t2", Addw { rd: T0, rs1: T1, rs2: T2 }),
    //     subw("subw	t0,t1,t2", Subw { rd: T0, rs1: T1, rs2: T2 }),
    //     sllw("sllw	t0,t1,t2", Sllw { rd: T0, rs1: T1, rs2: T2 }),
    //     srlw("srlw	t0,t1,t2", Srlw { rd: T0, rs1: T1, rs2: T2 }),
    //     sraw("sraw	t0,t1,t2", Sraw { rd: T0, rs1: T1, rs2: T2 }),

    //     // RV32M (8 tests)
    //     mul("mul	t0,t1,t2", Mul { rd: T0, rs1: T1, rs2: T2 }),
    //     mulh("mulh	t0,t1,t2", Mulh { rd: T0, rs1: T1, rs2: T2 }),
    //     mulhsu("mulhsu	t0,t1,t2", Mulhsu { rd: T0, rs1: T1, rs2: T2 }),
    //     mulhu("mulhu	t0,t1,t2", Mulhu { rd: T0, rs1: T1, rs2: T2 }),
    //     div("div	t0,t1,t2", Div { rd: T0, rs1: T1, rs2: T2 }),
    //     divu("divu	t0,t1,t2", Divu { rd: T0, rs1: T1, rs2: T2 }),
    //     rem("rem	t0,t1,t2", Rem { rd: T0, rs1: T1, rs2: T2 }),
    //     remu("remu	t0,t1,t2", Remu { rd: T0, rs1: T1, rs2: T2 }),

    //     // RV64M (5 tests)
    //     mulw("mulw	t0,t1,t2", Mulw { rd: T0, rs1: T1, rs2: T2 }),
    //     divw("divw	t0,t1,t2", Divw { rd: T0, rs1: T1, rs2: T2 }),
    //     divuw("divuw	t0,t1,t2", Divuw { rd: T0, rs1: T1, rs2: T2 }),
    //     remw("remw	t0,t1,t2", Remw { rd: T0, rs1: T1, rs2: T2 }),
    //     remuw("remuw	t0,t1,t2", Remuw { rd: T0, rs1: T1, rs2: T2 }),

    //     // RV32A (44 tests)
        // lr_w("lr.w	t0,(a0)", LrW { ord: Ordering::None, rd: Register::T0, rs1: A0 }),
    //     lr_w_aq("lr.w.aq	t0,(a0)", LrW { ord: Aq, rd: T0, rs1: A0 }),
    //     lr_w_rl("lr.w.rl	t0,(a0)", LrW { ord: Rl, rd: T0, rs1: A0 }),
    //     lr_w_aqrl("lr.w.aqrl	t0,(a0)", LrW { ord: Aqrl, rd: T0, rs1: A0 }),
    //     sc_w("sc.w	t0,a2,(a0)", ScW { ord: Empty, rd: T0, rs2: A2, rs1: A0 }),
    //     sc_w_aq("sc.w.aq	t0,a2,(a0)", ScW { ord: Aq, rd: T0, rs2: A2, rs1: A0 }),
    //     sc_w_rl("sc.w.rl	t0,a2,(a0)", ScW { ord: Rl, rd: T0, rs2: A2, rs1: A0 }),
    //     sc_w_aqrl("sc.w.aqrl	t0,a2,(a0)", ScW { ord: Aqrl, rd: T0, rs2: A2, rs1: A0 }),
    //     amoswap_w("amoswap.w	t1,t0,(a0)", AmoswapW { ord: Empty, rd: T1, rs2: T0, rs1: A0 }),
    //     amoswap_w_aq("amoswap.w.aq	t1,t0,(a0)", AmoswapW { ord: Aq, rd: T1, rs2: T0, rs1: A0 }),
    //     amoswap_w_rl("amoswap.w.rl	t1,t0,(a0)", AmoswapW { ord: Rl, rd: T1, rs2: T0, rs1: A0 }),
    //     amoswap_w_aqrl("amoswap.w.aqrl	t1,t0,(a0)", AmoswapW { ord: Aqrl, rd: T1, rs2: T0, rs1: A0 }),
    //     amoadd_w("amoadd.w	t1,t0,(a0)", AmoaddW { ord: Empty, rd: T1, rs2: T0, rs1: A0 }),
    //     amoadd_w_aq("amoadd.w.aq	t1,t0,(a0)", AmoaddW { ord: Aq, rd: T1, rs2: T0, rs1: A0 }),
    //     amoadd_w_rl("amoadd.w.rl	t1,t0,(a0)", AmoaddW { ord: Rl, rd: T1, rs2: T0, rs1: A0 }),
    //     amoadd_w_aqrl("amoadd.w.aqrl	t1,t0,(a0)", AmoaddW { ord: Aqrl, rd: T1, rs2: T0, rs1: A0 }),
    //     amoxor_w("amoxor.w	t1,t0,(a0)", AmoxorW { ord: Empty, rd: T1, rs2: T0, rs1: A0 }),
    //     amoxor_w_aq("amoxor.w.aq	t1,t0,(a0)", AmoxorW { ord: Aq, rd: T1, rs2: T0, rs1: A0 }),
    //     amoxor_w_rl("amoxor.w.rl	t1,t0,(a0)", AmoxorW { ord: Rl, rd: T1, rs2: T0, rs1: A0 }),
    //     amoxor_w_aqrl("amoxor.w.aqrl	t1,t0,(a0)", AmoxorW { ord: Aqrl, rd: T1, rs2: T0, rs1: A0 }),
    //     amoand_w("amoand.w	t1,t0,(a0)", AmoandW { ord: Empty, rd: T1, rs2: T0, rs1: A0 }),
    //     amoand_w_aq("amoand.w.aq	t1,t0,(a0)", AmoandW { ord: Aq, rd: T1, rs2: T0, rs1: A0 }),
    //     amoand_w_rl("amoand.w.rl	t1,t0,(a0)", AmoandW { ord: Rl, rd: T1, rs2: T0, rs1: A0 }),
    //     amoand_w_aqrl("amoand.w.aqrl	t1,t0,(a0)", AmoandW { ord: Aqrl, rd: T1, rs2: T0, rs1: A0 }),
    //     amoor_w("amoor.w	t1,t0,(a0)", AmoorW { ord: Empty, rd: T1, rs2: T0, rs1: A0 }),
    //     amoor_w_aq("amoor.w.aq	t1,t0,(a0)", AmoorW { ord: Aq, rd: T1, rs2: T0, rs1: A0 }),
    //     amoor_w_rl("amoor.w.rl	t1,t0,(a0)", AmoorW { ord: Rl, rd: T1, rs2: T0, rs1: A0 }),
    //     amoor_w_aqrl("amoor.w.aqrl	t1,t0,(a0)", AmoorW { ord: Aqrl, rd: T1, rs2: T0, rs1: A0 }),
    //     amomin_w("amomin.w	t1,t0,(a0)", AmominW { ord: Empty, rd: T1, rs2: T0, rs1: A0 }),
    //     amomin_w_aq("amomin.w.aq	t1,t0,(a0)", AmominW { ord: Aq, rd: T1, rs2: T0, rs1: A0 }),
    //     amomin_w_rl("amomin.w.rl	t1,t0,(a0)", AmominW { ord: Rl, rd: T1, rs2: T0, rs1: A0 }),
    //     amomin_w_aqrl("amomin.w.aqrl	t1,t0,(a0)", AmominW { ord: Aqrl, rd: T1, rs2: T0, rs1: A0 }),
    //     amomax_w("amomax.w	t1,t0,(a0)", AmomaxW { ord: Empty, rd: T1, rs2: T0, rs1: A0 }),
    //     amomax_w_aq("amomax.w.aq	t1,t0,(a0)", AmomaxW { ord: Aq, rd: T1, rs2: T0, rs1: A0 }),
    //     amomax_w_rl("amomax.w.rl	t1,t0,(a0)", AmomaxW { ord: Rl, rd: T1, rs2: T0, rs1: A0 }),
    //     amomax_w_aqrl("amomax.w.aqrl	t1,t0,(a0)", AmomaxW { ord: Aqrl, rd: T1, rs2: T0, rs1: A0 }),
    //     amominu_w("amominu.w	t1,t0,(a0)", AmominuW { ord: Empty, rd: T1, rs2: T0, rs1: A0 }),
    //     amominu_w_aq("amominu.w.aq	t1,t0,(a0)", AmominuW { ord: Aq, rd: T1, rs2: T0, rs1: A0 }),
    //     amominu_w_rl("amominu.w.rl	t1,t0,(a0)", AmominuW { ord: Rl, rd: T1, rs2: T0, rs1: A0 }),
    //     amominu_w_aqrl("amominu.w.aqrl	t1,t0,(a0)", AmominuW { ord: Aqrl, rd: T1, rs2: T0, rs1: A0 }),
    //     amomaxu_w("amomaxu.w	t1,t0,(a0)", AmomaxuW { ord: Empty, rd: T1, rs2: T0, rs1: A0 }),
    //     amomaxu_w_aq("amomaxu.w.aq	t1,t0,(a0)", AmomaxuW { ord: Aq, rd: T1, rs2: T0, rs1: A0 }),
    //     amomaxu_w_rl("amomaxu.w.rl	t1,t0,(a0)", AmomaxuW { ord: Rl, rd: T1, rs2: T0, rs1: A0 }),
    //     amomaxu_w_aqrl("amomaxu.w.aqrl	t1,t0,(a0)", AmomaxuW { ord: Aqrl, rd: T1, rs2: T0, rs1: A0 }),

    //     // RV64A (44 tests)
    //     lr_d("lr.d	t0,(a0)", LrD { ord: Empty, rd: T0, rs1: A0 }),
    //     lr_d_aq("lr.d.aq	t0,(a0)", LrD { ord: Aq, rd: T0, rs1: A0 }),
    //     lr_d_rl("lr.d.rl	t0,(a0)", LrD { ord: Rl, rd: T0, rs1: A0 }),
    //     lr_d_aqrl("lr.d.aqrl	t0,(a0)", LrD { ord: Aqrl, rd: T0, rs1: A0 }),
    //     sc_d("sc.d	t0,a2,(a0)", ScD { ord: Empty, rd: T0, rs2: A2, rs1: A0 }),
    //     sc_d_aq("sc.d.aq	t0,a2,(a0)", ScD { ord: Aq, rd: T0, rs2: A2, rs1: A0 }),
    //     sc_d_rl("sc.d.rl	t0,a2,(a0)", ScD { ord: Rl, rd: T0, rs2: A2, rs1: A0 }),
    //     sc_d_aqrl("sc.d.aqrl	t0,a2,(a0)", ScD { ord: Aqrl, rd: T0, rs2: A2, rs1: A0 }),
    //     amoswap_d("amoswap.d	t1,t0,(a0)", AmoswapD { ord: Empty, rd: T1, rs2: T0, rs1: A0 }),
    //     amoswap_d_aq("amoswap.d.aq	t1,t0,(a0)", AmoswapD { ord: Aq, rd: T1, rs2: T0, rs1: A0 }),
    //     amoswap_d_rl("amoswap.d.rl	t1,t0,(a0)", AmoswapD { ord: Rl, rd: T1, rs2: T0, rs1: A0 }),
    //     amoswap_d_aqrl("amoswap.d.aqrl	t1,t0,(a0)", AmoswapD { ord: Aqrl, rd: T1, rs2: T0, rs1: A0 }),
    //     amoadd_d("amoadd.d	t1,t0,(a0)", AmoaddD { ord: Empty, rd: T1, rs2: T0, rs1: A0 }),
    //     amoadd_d_aq("amoadd.d.aq	t1,t0,(a0)", AmoaddD { ord: Aq, rd: T1, rs2: T0, rs1: A0 }),
    //     amoadd_d_rl("amoadd.d.rl	t1,t0,(a0)", AmoaddD { ord: Rl, rd: T1, rs2: T0, rs1: A0 }),
    //     amoadd_d_aqrl("amoadd.d.aqrl	t1,t0,(a0)", AmoaddD { ord: Aqrl, rd: T1, rs2: T0, rs1: A0 }),
    //     amoxor_d("amoxor.d	t1,t0,(a0)", AmoxorD { ord: Empty, rd: T1, rs2: T0, rs1: A0 }),
    //     amoxor_d_aq("amoxor.d.aq	t1,t0,(a0)", AmoxorD { ord: Aq, rd: T1, rs2: T0, rs1: A0 }),
    //     amoxor_d_rl("amoxor.d.rl	t1,t0,(a0)", AmoxorD { ord: Rl, rd: T1, rs2: T0, rs1: A0 }),
    //     amoxor_d_aqrl("amoxor.d.aqrl	t1,t0,(a0)", AmoxorD { ord: Aqrl, rd: T1, rs2: T0, rs1: A0 }),
    //     amoand_d("amoand.d	t1,t0,(a0)", AmoandD { ord: Empty, rd: T1, rs2: T0, rs1: A0 }),
    //     amoand_d_aq("amoand.d.aq	t1,t0,(a0)", AmoandD { ord: Aq, rd: T1, rs2: T0, rs1: A0 }),
    //     amoand_d_rl("amoand.d.rl	t1,t0,(a0)", AmoandD { ord: Rl, rd: T1, rs2: T0, rs1: A0 }),
    //     amoand_d_aqrl("amoand.d.aqrl	t1,t0,(a0)", AmoandD { ord: Aqrl, rd: T1, rs2: T0, rs1: A0 }),
    //     amoor_d("amoor.d	t1,t0,(a0)", AmoorD { ord: Empty, rd: T1, rs2: T0, rs1: A0 }),
    //     amoor_d_aq("amoor.d.aq	t1,t0,(a0)", AmoorD { ord: Aq, rd: T1, rs2: T0, rs1: A0 }),
    //     amoor_d_rl("amoor.d.rl	t1,t0,(a0)", AmoorD { ord: Rl, rd: T1, rs2: T0, rs1: A0 }),
    //     amoor_d_aqrl("amoor.d.aqrl	t1,t0,(a0)", AmoorD { ord: Aqrl, rd: T1, rs2: T0, rs1: A0 }),
    //     amomin_d("amomin.d	t1,t0,(a0)", AmominD { ord: Empty, rd: T1, rs2: T0, rs1: A0 }),
    //     amomin_d_aq("amomin.d.aq	t1,t0,(a0)", AmominD { ord: Aq, rd: T1, rs2: T0, rs1: A0 }),
    //     amomin_d_rl("amomin.d.rl	t1,t0,(a0)", AmominD { ord: Rl, rd: T1, rs2: T0, rs1: A0 }),
    //     amomin_d_aqrl("amomin.d.aqrl	t1,t0,(a0)", AmominD { ord: Aqrl, rd: T1, rs2: T0, rs1: A0 }),
    //     amomax_d("amomax.d	t1,t0,(a0)", AmomaxD { ord: Empty, rd: T1, rs2: T0, rs1: A0 }),
    //     amomax_d_aq("amomax.d.aq	t1,t0,(a0)", AmomaxD { ord: Aq, rd: T1, rs2: T0, rs1: A0 }),
    //     amomax_d_rl("amomax.d.rl	t1,t0,(a0)", AmomaxD { ord: Rl, rd: T1, rs2: T0, rs1: A0 }),
    //     amomax_d_aqrl("amomax.d.aqrl	t1,t0,(a0)", AmomaxD { ord: Aqrl, rd: T1, rs2: T0, rs1: A0 }),
    //     amominu_d("amominu.d	t1,t0,(a0)", AmominuD { ord: Empty, rd: T1, rs2: T0, rs1: A0 }),
    //     amominu_d_aq("amominu.d.aq	t1,t0,(a0)", AmominuD { ord: Aq, rd: T1, rs2: T0, rs1: A0 }),
    //     amominu_d_rl("amominu.d.rl	t1,t0,(a0)", AmominuD { ord: Rl, rd: T1, rs2: T0, rs1: A0 }),
    //     amominu_d_aqrl("amominu.d.aqrl	t1,t0,(a0)", AmominuD { ord: Aqrl, rd: T1, rs2: T0, rs1: A0 }),
    //     amomaxu_d("amomaxu.d	t1,t0,(a0)", AmomaxuD { ord: Empty, rd: T1, rs2: T0, rs1: A0 }),
    //     amomaxu_d_aq("amomaxu.d.aq	t1,t0,(a0)", AmomaxuD { ord: Aq, rd: T1, rs2: T0, rs1: A0 }),
    //     amomaxu_d_rl("amomaxu.d.rl	t1,t0,(a0)", AmomaxuD { ord: Rl, rd: T1, rs2: T0, rs1: A0 }),
    //     amomaxu_d_aqrl("amomaxu.d.aqrl	t1,t0,(a0)", AmomaxuD { ord: Aqrl, rd: T1, rs2: T0, rs1: A0 }),

    //     // RV32F (39 tests)
    //     flw("flw	fa0,-20(s0)", Flw { rd: Fa0, imm: (-20).into(), rs1: S0 }),
    //     fsw("fsw	fa0,-20(s0)", Fsw { rs2: Fa0, imm: (-20).into(), rs1: S0 }),
    //     fmadd_s("fmadd.s	fa0,fa0,fa1,fa2", FmaddS { rd: Fa0, rs1: Fa0, rs2: Fa1, rs3: Fa2 }),
    //     fmadd_s_rm("fmadd.s	fa0,fa0,fa1,fa2,rtz", FmaddS { rd: Fa0, rs1: Fa0, rs2: Fa1, rs3: Fa2 }),
    //     fmsub_s("fmsub.s	fa0,fa0,fa1,fa2", FmsubS { rd: Fa0, rs1: Fa0, rs2: Fa1, rs3: Fa2 }),
    //     fmsub_s_rm("fmsub.s	fa0,fa0,fa1,fa2,rtz", FmsubS { rd: Fa0, rs1: Fa0, rs2: Fa1, rs3: Fa2 }),
    //     fnmsub_s("fnmsub.s	fa0,fa0,fa1,fa2", FnmsubS { rd: Fa0, rs1: Fa0, rs2: Fa1, rs3: Fa2 }),
    //     fnmsub_s_rm("fnmsub.s	fa0,fa0,fa1,fa2,rtz", FnmsubS { rd: Fa0, rs1: Fa0, rs2: Fa1, rs3: Fa2 }),
    //     fnmadd_s("fnmadd.s	fa0,fa0,fa1,fa2", FnmaddS { rd: Fa0, rs1: Fa0, rs2: Fa1, rs3: Fa2 }),
    //     fnmadd_s_rm("fnmadd.s	fa0,fa0,fa1,fa2,rtz", FnmaddS { rd: Fa0, rs1: Fa0, rs2: Fa1, rs3: Fa2 }),
    //     fadd_s("fadd.s	fa3,fa4,fa5", FaddS { rd: Fa3, rs1: Fa4, rs2: Fa5 }),
    //     fadd_s_rm("fadd.s	fa3,fa4,fa5,rtz", FaddS { rd: Fa3, rs1: Fa4, rs2: Fa5 }),
    //     fsub_s("fsub.s	fa3,fa4,fa5", FsubS { rd: Fa3, rs1: Fa4, rs2: Fa5 }),
    //     fsub_s_rm("fsub.s	fa3,fa4,fa5,rtz", FsubS { rd: Fa3, rs1: Fa4, rs2: Fa5 }),
    //     fmul_s("fmul.s	fa3,fa4,fa5", FmulS { rd: Fa3, rs1: Fa4, rs2: Fa5 }),
    //     fmul_s_rm("fmul.s	fa3,fa4,fa5,rtz", FmulS { rd: Fa3, rs1: Fa4, rs2: Fa5 }),
    //     fdiv_s("fdiv.s	fa3,fa4,fa5", FdivS { rd: Fa3, rs1: Fa4, rs2: Fa5 }),
    //     fdiv_s_rm("fdiv.s	fa3,fa4,fa5,rtz", FdivS { rd: Fa3, rs1: Fa4, rs2: Fa5 }),
    //     fsqrt_s("fsqrt.s	fa0,fa1", FsqrtS { rd: Fa0, rs1: Fa1 }),
    //     fsqrt_s_rm("fsqrt.s	fa0,fa1,rtz", FsqrtS { rd: Fa0, rs1: Fa1 }),
    //     fsgnj_s("fsgnj.s	ft0,ft1,ft2", FsgnjS { rd: Ft0, rs1: Ft1, rs2: Ft2 }),
    //     fsgnjn_s("fsgnjn.s	ft0,ft1,ft2", FsgnjnS { rd: Ft0, rs1: Ft1, rs2: Ft2 }),
    //     fsgnjx_s("fsgnjx.s	ft0,ft1,ft2", FsgnjxS { rd: Ft0, rs1: Ft1, rs2: Ft2 }),
    //     fmin_s("fmin.s	ft0,ft1,ft2", FminS { rd: Ft0, rs1: Ft1, rs2: Ft2 }),
    //     fmax_s("fmax.s	ft0,ft1,ft2", FmaxS { rd: Ft0, rs1: Ft1, rs2: Ft2 }),
    //     fcvt_w_s("fcvt.w.s	a5,fa5", FcvtWS { rd: A5, rs1: Fa5 }),
    //     fcvt_w_s_rm("fcvt.w.s	a5,fa5,rtz", FcvtWS { rd: A5, rs1: Fa5 }),
    //     fcvt_wu_s("fcvt.wu.s	a5,fa5", FcvtWuS { rd: A5, rs1: Fa5 }),
    //     fcvt_wu_s_rm("fcvt.wu.s	a5,fa5,rtz", FcvtWuS { rd: A5, rs1: Fa5 }),
    //     fmv_x_w("fmv.x.w	t0,ft0", FmvXW { rd: T0, rs1: Ft0 }),
    //     feq_s("feq.s	a5,fa4,fa5", FeqS { rd: A5, rs1: Fa4, rs2: Fa5 }),
    //     flt_s("flt.s	a5,fa4,fa5", FltS { rd: A5, rs1: Fa4, rs2: Fa5 }),
    //     fle_s("fle.s	a5,fa4,fa5", FleS { rd: A5, rs1: Fa4, rs2: Fa5 }),
    //     fclass_s("fclass.s	t0,ft0", FclassS { rd: T0, rs1: Ft0 }),
    //     fcvt_s_w("fcvt.s.w	fa5,a5", FcvtSW { rd: Fa5, rs1: A5 }),
    //     fcvt_s_w_rm("fcvt.s.w	fa5,a5,rtz", FcvtSW { rd: Fa5, rs1: A5 }),
    //     fcvt_s_wu("fcvt.s.wu	fa5,a5", FcvtSWu { rd: Fa5, rs1: A5 }),
    //     fcvt_s_wu_rm("fcvt.s.wu	fa5,a5,rtz", FcvtSWu { rd: Fa5, rs1: A5 }),
    //     fmv_w_x("fmv.w.x	ft0,t0", FmvWX { rd: Ft0, rs1: T0 }),

    //     // RV64F (8 tests)
    //     fcvt_l_s("fcvt.l.s	a5,fa5", FcvtLS { rd: A5, rs1: Fa5 }),
    //     fcvt_l_s_rm("fcvt.l.s	a5,fa5,rtz", FcvtLS { rd: A5, rs1: Fa5 }),
    //     fcvt_lu_s("fcvt.lu.s	a5,fa5", FcvtLuS { rd: A5, rs1: Fa5 }),
    //     fcvt_lu_s_rm("fcvt.lu.s	a5,fa5,rtz", FcvtLuS { rd: A5, rs1: Fa5 }),
    //     fcvt_s_l("fcvt.s.l	fa5,a5", FcvtSL { rd: Fa5, rs1: A5 }),
    //     fcvt_s_l_rm("fcvt.s.l	fa5,a5,rtz", FcvtSL { rd: Fa5, rs1: A5 }),
    //     fcvt_s_lu("fcvt.s.lu	fa5,a5", FcvtSLu { rd: Fa5, rs1: A5 }),
    //     fcvt_s_lu_rm("fcvt.s.lu	fa5,a5,rtz", FcvtSLu { rd: Fa5, rs1: A5 }),

    //     // RV32D (38 tests)
    //     fld("fld	fa4,-24(s0)", Fld { rd: Fa4, imm: (-24).into(), rs1: S0 }),
    //     fsd("fsd	fa0,-24(s0)", Fsd { rs2: Fa0, imm: (-24).into(), rs1: S0 }),
    //     fmadd_d("fmadd.d	fa0,fa0,fa1,fa2", FmaddD { rd: Fa0, rs1: Fa0, rs2: Fa1, rs3: Fa2 }),
    //     fmadd_d_rm("fmadd.d	fa0,fa0,fa1,fa2,rtz", FmaddD { rd: Fa0, rs1: Fa0, rs2: Fa1, rs3: Fa2 }),
    //     fmsub_d("fmsub.d	fa0,fa0,fa1,fa2", FmsubD { rd: Fa0, rs1: Fa0, rs2: Fa1, rs3: Fa2 }),
    //     fmsub_d_rm("fmsub.d	fa0,fa0,fa1,fa2,rtz", FmsubD { rd: Fa0, rs1: Fa0, rs2: Fa1, rs3: Fa2 }),
    //     fnmsub_d("fnmsub.d	fa0,fa0,fa1,fa2", FnmsubD { rd: Fa0, rs1: Fa0, rs2: Fa1, rs3: Fa2 }),
    //     fnmsub_d_rm("fnmsub.d	fa0,fa0,fa1,fa2,rtz", FnmsubD { rd: Fa0, rs1: Fa0, rs2: Fa1, rs3: Fa2 }),
    //     fnmadd_d("fnmadd.d	fa0,fa0,fa1,fa2", FnmaddD { rd: Fa0, rs1: Fa0, rs2: Fa1, rs3: Fa2 }),
    //     fnmadd_d_rm("fnmadd.d	fa0,fa0,fa1,fa2,rtz", FnmaddD { rd: Fa0, rs1: Fa0, rs2: Fa1, rs3: Fa2 }),
    //     fadd_d("fadd.d	fa3,fa4,fa5", FaddD { rd: Fa3, rs1: Fa4, rs2: Fa5 }),
    //     fadd_d_rm("fadd.d	fa3,fa4,fa5,rtz", FaddD { rd: Fa3, rs1: Fa4, rs2: Fa5 }),
    //     fsub_d("fsub.d	fa3,fa4,fa5", FsubD { rd: Fa3, rs1: Fa4, rs2: Fa5 }),
    //     fsub_d_rm("fsub.d	fa3,fa4,fa5,rtz", FsubD { rd: Fa3, rs1: Fa4, rs2: Fa5 }),
    //     fmul_d("fmul.d	fa3,fa4,fa5", FmulD { rd: Fa3, rs1: Fa4, rs2: Fa5 }),
    //     fmul_d_rm("fmul.d	fa3,fa4,fa5,rtz", FmulD { rd: Fa3, rs1: Fa4, rs2: Fa5 }),
    //     fdiv_d("fdiv.d	fa3,fa4,fa5", FdivD { rd: Fa3, rs1: Fa4, rs2: Fa5 }),
    //     fdiv_d_rm("fdiv.d	fa3,fa4,fa5,rtz", FdivD { rd: Fa3, rs1: Fa4, rs2: Fa5 }),
    //     fsqrt_d("fsqrt.d	fa0,fa1", FsqrtD { rd: Fa0, rs1: Fa1 }),
    //     fsqrt_d_rm("fsqrt.d	fa0,fa1,rtz", FsqrtD { rd: Fa0, rs1: Fa1 }),
    //     fsgnj_d("fsgnj.d	ft0,ft1,ft2", FsgnjD { rd: Ft0, rs1: Ft1, rs2: Ft2 }),
    //     fsgnjn_d("fsgnjn.d	ft0,ft1,ft2", FsgnjnD { rd: Ft0, rs1: Ft1, rs2: Ft2 }),
    //     fsgnjx_d("fsgnjx.d	ft0,ft1,ft2", FsgnjxD { rd: Ft0, rs1: Ft1, rs2: Ft2 }),
    //     fmin_d("fmin.d	ft0,ft1,ft2", FminD { rd: Ft0, rs1: Ft1, rs2: Ft2 }),
    //     fmax_d("fmax.d	ft0,ft1,ft2", FmaxD { rd: Ft0, rs1: Ft1, rs2: Ft2 }),
    //     fcvt_s_d("fcvt.s.d	fa0,fa5", FcvtSD { rd: Fa0, rs1: Fa5 }),
    //     fcvt_s_d_rm("fcvt.s.d	fa0,fa5,rtz", FcvtSD { rd: Fa0, rs1: Fa5 }),
    //     fcvt_d_s("fcvt.d.s	fa0,fa5", FcvtDS { rd: Fa0, rs1: Fa5 }),
    //     // `FcvtDS` will never round.
    //     feq_d("feq.d	a5,fa4,fa5", FeqD { rd: A5, rs1: Fa4, rs2: Fa5 }),
    //     flt_d("flt.d	a5,fa4,fa5", FltD { rd: A5, rs1: Fa4, rs2: Fa5 }),
    //     fle_d("fle.d	a5,fa4,fa5", FleD { rd: A5, rs1: Fa4, rs2: Fa5 }),
    //     fclass_d("fclass.d	t0,ft0", FclassD { rd: T0, rs1: Ft0 }),
    //     fcvt_w_d("fcvt.w.d	a5,fa5", FcvtWD { rd: A5, rs1: Fa5 }),
    //     fcvt_w_d_rm("fcvt.w.d	a5,fa5,rtz", FcvtWD { rd: A5, rs1: Fa5 }),
    //     fcvt_wu_d("fcvt.wu.d	a5,fa5", FcvtWuD { rd: A5, rs1: Fa5 }),
    //     fcvt_wu_d_rm("fcvt.wu.d	a5,fa5,rtz", FcvtWuD { rd: A5, rs1: Fa5 }),
    //     fcvt_d_w("fcvt.d.w	fa5,a5", FcvtDW { rd: Fa5, rs1: A5 }),
    //     // `FcvtDW` always produces an exact result and is unaffected by the rounding mode.
    //     fcvt_d_wu("fcvt.d.wu	fa5,a5", FcvtDWu { rd: Fa5, rs1: A5 }),
    //     // `FcvtDWu` always produces an exact result and is unaffected by the rounding mode.

    //     // RV64D (10 tests)
    //     fcvt_l_d("fcvt.l.d	a5,fa5", FcvtLD { rd: A5, rs1: Fa5 }),
    //     fcvt_l_d_rm("fcvt.l.d	a5,fa5,rtz", FcvtLD { rd: A5, rs1: Fa5 }),
    //     fcvt_lu_d("fcvt.lu.d	a5,fa5", FcvtLuD { rd: A5, rs1: Fa5 }),
    //     fcvt_lu_d_rm("fcvt.lu.d	a5,fa5,rtz", FcvtLuD { rd: A5, rs1: Fa5 }),
    //     fmv_x_d("fmv.x.d	t0,ft0", FmvXD { rd: T0, rs1: Ft0 }),
    //     fcvt_d_l("fcvt.d.l	fa5,a5", FcvtDL { rd: Fa5, rs1: A5 }),
    //     fcvt_d_l_rm("fcvt.d.l	fa5,a5,rtz", FcvtDL { rd: Fa5, rs1: A5 }),
    //     fcvt_d_lu("fcvt.d.lu	fa5,a5", FcvtDLu { rd: Fa5, rs1: A5 }),
    //     fcvt_d_lu_rm("fcvt.d.lu	fa5,a5,rtz", FcvtDLu { rd: Fa5, rs1: A5 }),
    //     fmv_d_x("fmv.d.x	ft0,t0", FmvDX { rd: Ft0, rs1: T0 }),

    //     // Pseudoinstructions (26 tests)
    //     nop("nop", Nop {}),
    //     li("li	a5,0", Li { rd: A5, imm: 0.into() }),
    //     mv("mv	a5,a0", Mv { rd: A5, rs1: A0 }),
    //     not("not	a4,a5", Not { rd: A4, rs1: A5 }),
    //     neg("neg	a4,a5", Neg { rd: A4, rs1: A5 }),
    //     negw("negw	a4,a5", Negw { rd: A4, rs1: A5 }),
    //     sext_w("sext.w	a4,a5", SextW { rd: A4, rs1: A5 }),
    //     seqz("seqz	a4,a5", Seqz { rd: A4, rs1: A5 }),
    //     snez("snez	a4,a5", Snez { rd: A4, rs1: A5 }),
    //     sltz("sltz	a4,a5", Sltz { rd: A4, rs1: A5 }),
    //     sgtz("sgtz	a4,a5", Sgtz { rd: A4, rs1: A5 }),

    //     fmv_s("fmv.s	fa0,fa5", FmvS { rd: Fa0, rs1: Fa5 }),
    //     fabs_s("fabs.s	fa0,fa5", FabsS { rd: Fa0, rs1: Fa5 }),
    //     fneg_s("fneg.s	fa0,fa5", FnegS { rd: Fa0, rs1: Fa5 }),
    //     fmv_d("fmv.d	fa0,fa5", FmvD { rd: Fa0, rs1: Fa5 }),
    //     fabs_d("fabs.d	fa0,fa5", FabsD { rd: Fa0, rs1: Fa5 }),
    //     fneg_d("fneg.d	fa0,fa5", FnegD { rd: Fa0, rs1: Fa5 }),

    //     beqz("beqz	a5,104c6", Beqz { rs1: A5, addr: 0x104c6.into() }),
    //     bnez("bnez	a5,104c6", Bnez { rs1: A5, addr: 0x104c6.into() }),
    //     blez("blez	a5,104c6", Blez { rs1: A5, addr: 0x104c6.into() }),
    //     bgez("bgez	a5,106c6", Bgez { rs1: A5, addr: 0x106c6.into() }),
    //     bltz("bltz	a5,106be", Bltz { rs1: A5, addr: 0x106be.into() }),
    //     bgtz("bgtz	s0,105e2", Bgtz { rs1: S0, addr: 0x105e2.into() }),

    //     j("j	106f2", J { addr: 0x106f2.into() }),
    //     jr("jr	a5", Jr { rs1: A5 }),
    //     ret("ret", Ret {}),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::CfgBuilder;
//     use crate::cfg::*;
//     use crate::riscv_isa::RiscvInstruction::*;
//     use crate::riscv_isa::RiscvRegister::*;
//     use crate::riscv_parser;
//     use std::collections::HashMap;

//     #[test]
//     fn minimal() {
//         let source = "
//             Disassembly of section .text:

//             00000000000105c4 <main>:
//                 105c4:	8082                	ret
//         ";
//         let indirect_targets = riscv_parser::parse_rodata(source);
//         let riscv_insts = riscv_parser::parse_text(source);
//         let cfg = CfgBuilder::new(riscv_insts, indirect_targets).run();
//         let expected = vec![RiscvFunction {
//             name: String::from("main"),
//             basic_blocks: vec![BasicBlock {
//                 instructions: vec![Ret {
//                     label: Some(String::from("main")),
//                     address: 0x105c4.into(),
//                     comment: None,
//                 }],
//                 continue_target: None,
//                 jump_target: None,
//             }],
//             indirect_targets: HashMap::new(),
//         }];
//         assert_eq!(cfg, expected);
//     }

//     #[test]
//     fn functions() {
//         let source = "
//             Disassembly of section .text:

//             0000000000010506 <f>:
//                 1051c:	8082                	ret

//             000000000001051e <main>:
//                 1052a:	ff8080e7          	jalr	-8(ra) # 1051e <main>
//                 10530:	fdbff0ef          	jal	ra,104e0 <f>
//                 10534:	8082                	ret
//         ";
//         let indirect_targets = riscv_parser::parse_rodata(source);
//         let riscv_insts = riscv_parser::parse_text(source);
//         let cfg = CfgBuilder::new(riscv_insts, indirect_targets).run();
//         let expected = vec![
//             RiscvFunction {
//                 name: String::from("f"),
//                 basic_blocks: vec![BasicBlock {
//                     instructions: vec![Ret {
//                         label: Some(String::from("f")),
//                         address: 0x1051c.into(),
//                         comment: None,
//                     }],
//                     continue_target: None,
//                     jump_target: None,
//                 }],
//                 indirect_targets: HashMap::new(),
//             },
//             RiscvFunction {
//                 name: String::from("main"),
//                 basic_blocks: vec![BasicBlock {
//                     instructions: vec![
//                         Jalr {
//                             label: Some(String::from("main")),
//                             address: 0x1052a.into(),
//                             rd: Ra,
//                             imm: (-8).into(),
//                             rs1: Ra,
//                             comment: Some(String::from("# 1051e <main>")),
//                         },
//                         Jal {
//                             label: None,
//                             address: 0x10530.into(),
//                             rd: Ra,
//                             addr: 0x104e0.into(),
//                             comment: Some(String::from("<f>")),
//                         },
//                         Ret {
//                             label: None,
//                             address: 0x10534.into(),
//                             comment: None,
//                         },
//                     ],
//                     continue_target: None,
//                     jump_target: None,
//                 }],
//                 indirect_targets: HashMap::new(),
//             },
//         ];
//         assert_eq!(cfg, expected);
//     }

//     #[test]
//     fn branches() {
//         let source = "
//             Disassembly of section .text:

//             00000000000104f8 <main>:
//                 104f8:	fe528de3          	beq	t0,t0,104f8 <main>
//                 10502:	fe529be3          	bne	t0,t0,104f8 <main>
//                 10506:	fe52c9e3          	blt	t0,t0,104f8 <main>
//                 1050a:	fe52d7e3          	bge	t0,t0,104f8 <main>
//                 1050e:	fe52e5e3          	bltu	t0,t0,104f8 <main>
//                 10512:	fe52f3e3          	bgeu	t0,t0,104f8 <main>
//                 10516:	fe0281e3          	beqz	t0,104f8 <main>
//                 1051a:	fc029fe3          	bnez	t0,104f8 <main>
//                 1051e:	fc505de3          	blez	t0,104f8 <main>
//                 10522:	fc02dbe3          	bgez	t0,104f8 <main>
//                 10526:	fc02c9e3          	bltz	t0,104f8 <main>
//                 1052a:	fc5047e3          	bgtz	t0,104f8 <main>
//                 10536:	8082                	ret
//         ";
//         let indirect_targets = riscv_parser::parse_rodata(source);
//         let riscv_insts = riscv_parser::parse_text(source);
//         let cfg = CfgBuilder::new(riscv_insts, indirect_targets).run();
//         let expected = vec![RiscvFunction {
//             name: String::from("main"),
//             basic_blocks: vec![
//                 BasicBlock {
//                     instructions: vec![Beq {
//                         label: Some(String::from("main")),
//                         address: 0x104f8.into(),
//                         rs1: T0,
//                         rs2: T0,
//                         addr: 0x104f8.into(),
//                         comment: Some(String::from("<main>")),
//                     }],
//                     continue_target: Some(1),
//                     jump_target: Some(0),
//                 },
//                 BasicBlock {
//                     instructions: vec![Bne {
//                         label: None,
//                         address: 0x10502.into(),
//                         rs1: T0,
//                         rs2: T0,
//                         addr: 0x104f8.into(),
//                         comment: Some(String::from("<main>")),
//                     }],
//                     continue_target: Some(2),
//                     jump_target: Some(0),
//                 },
//                 BasicBlock {
//                     instructions: vec![Blt {
//                         label: None,
//                         address: 0x10506.into(),
//                         rs1: T0,
//                         rs2: T0,
//                         addr: 0x104f8.into(),
//                         comment: Some(String::from("<main>")),
//                     }],
//                     continue_target: Some(3),
//                     jump_target: Some(0),
//                 },
//                 BasicBlock {
//                     instructions: vec![Bge {
//                         label: None,
//                         address: 0x1050a.into(),
//                         rs1: T0,
//                         rs2: T0,
//                         addr: 0x104f8.into(),
//                         comment: Some(String::from("<main>")),
//                     }],
//                     continue_target: Some(4),
//                     jump_target: Some(0),
//                 },
//                 BasicBlock {
//                     instructions: vec![Bltu {
//                         label: None,
//                         address: 0x1050e.into(),
//                         rs1: T0,
//                         rs2: T0,
//                         addr: 0x104f8.into(),
//                         comment: Some(String::from("<main>")),
//                     }],
//                     continue_target: Some(5),
//                     jump_target: Some(0),
//                 },
//                 BasicBlock {
//                     instructions: vec![Bgeu {
//                         label: None,
//                         address: 0x10512.into(),
//                         rs1: T0,
//                         rs2: T0,
//                         addr: 0x104f8.into(),
//                         comment: Some(String::from("<main>")),
//                     }],
//                     continue_target: Some(6),
//                     jump_target: Some(0),
//                 },
//                 BasicBlock {
//                     instructions: vec![Beqz {
//                         label: None,
//                         address: 0x10516.into(),
//                         rs1: T0,
//                         addr: 0x104f8.into(),
//                         comment: Some(String::from("<main>")),
//                     }],
//                     continue_target: Some(7),
//                     jump_target: Some(0),
//                 },
//                 BasicBlock {
//                     instructions: vec![Bnez {
//                         label: None,
//                         address: 0x1051a.into(),
//                         rs1: T0,
//                         addr: 0x104f8.into(),
//                         comment: Some(String::from("<main>")),
//                     }],
//                     continue_target: Some(8),
//                     jump_target: Some(0),
//                 },
//                 BasicBlock {
//                     instructions: vec![Blez {
//                         label: None,
//                         address: 0x1051e.into(),
//                         rs1: T0,
//                         addr: 0x104f8.into(),
//                         comment: Some(String::from("<main>")),
//                     }],
//                     continue_target: Some(9),
//                     jump_target: Some(0),
//                 },
//                 BasicBlock {
//                     instructions: vec![Bgez {
//                         label: None,
//                         address: 0x10522.into(),
//                         rs1: T0,
//                         addr: 0x104f8.into(),
//                         comment: Some(String::from("<main>")),
//                     }],
//                     continue_target: Some(10),
//                     jump_target: Some(0),
//                 },
//                 BasicBlock {
//                     instructions: vec![Bltz {
//                         label: None,
//                         address: 0x10526.into(),
//                         rs1: T0,
//                         addr: 0x104f8.into(),
//                         comment: Some(String::from("<main>")),
//                     }],
//                     continue_target: Some(11),
//                     jump_target: Some(0),
//                 },
//                 BasicBlock {
//                     instructions: vec![Bgtz {
//                         label: None,
//                         address: 0x1052a.into(),
//                         rs1: T0,
//                         addr: 0x104f8.into(),
//                         comment: Some(String::from("<main>")),
//                     }],
//                     continue_target: Some(12),
//                     jump_target: Some(0),
//                 },
//                 BasicBlock {
//                     instructions: vec![Ret {
//                         label: None,
//                         address: 0x10536.into(),
//                         comment: None,
//                     }],
//                     continue_target: None,
//                     jump_target: None,
//                 },
//             ],
//             indirect_targets: HashMap::new(),
//         }];
//         assert_eq!(cfg, expected);
//     }

//     #[test]
//     fn indirect_jumps() {
//         let source = "
//             Disassembly of section .text:

//             00000000000104e0 <f>:
//                 104e0:	8782                	jr	a5
//                 1050e:	fec42783          	lw	a5,-20(s0)
//                 10512:	2785                	addiw	a5,a5,1
//                 10514:	fef42623          	sw	a5,-20(s0)
//                 10518:	a80d                	j	1054a <f+0x6a>
//                 1053e:	fec42783          	lw	a5,-20(s0)
//                 10542:	2795                	addiw	a5,a5,5
//                 10544:	fef42623          	sw	a5,-20(s0)
//                 10548:	0001                	nop
//                 1054a:	8082                	ret

//             0000000000010556 <main>:
//                 10556:	f81ff0ef          	jal	ra,104e0 <f>
//                 1056e:	8082                	ret

//             Disassembly of section .rodata:

//             00000000000105cc <.rodata>:
//                 105cc:	054a                	slli	a0,a0,0x12
//                 105ce:	0001                	nop
//                 105d0:	050e                	slli	a0,a0,0x3
//                 105d2:	0001                	nop
//                 105e0:	053e                	slli	a0,a0,0xf
//                 105e2:	0001                	nop
//         ";
//         let indirect_targets = riscv_parser::parse_rodata(source);
//         let riscv_insts = riscv_parser::parse_text(source);
//         let cfg = CfgBuilder::new(riscv_insts, indirect_targets).run();
//         let expected = vec![
//             RiscvFunction {
//                 name: String::from("f"),
//                 basic_blocks: vec![
//                     BasicBlock {
//                         instructions: vec![Jr {
//                             label: Some(String::from("f")),
//                             address: 0x104e0.into(),
//                             rs1: A5,
//                             comment: None,
//                         }],
//                         continue_target: None,
//                         jump_target: None,
//                     },
//                     BasicBlock {
//                         instructions: vec![
//                             Lw {
//                                 label: None,
//                                 address: 0x1050e.into(),
//                                 rd: A5,
//                                 imm: (-20).into(),
//                                 rs1: S0,
//                                 comment: None,
//                             },
//                             Addiw {
//                                 label: None,
//                                 address: 0x10512.into(),
//                                 rd: A5,
//                                 rs1: A5,
//                                 imm: 1.into(),
//                                 comment: None,
//                             },
//                             Sw {
//                                 label: None,
//                                 address: 0x10514.into(),
//                                 rs2: A5,
//                                 imm: (-20).into(),
//                                 rs1: S0,
//                                 comment: None,
//                             },
//                             J {
//                                 label: None,
//                                 address: 0x10518.into(),
//                                 addr: 0x1054a.into(),
//                                 comment: Some(String::from("<f+0x6a>")),
//                             },
//                         ],
//                         continue_target: None,
//                         jump_target: Some(3),
//                     },
//                     BasicBlock {
//                         instructions: vec![
//                             Lw {
//                                 label: None,
//                                 address: 0x1053e.into(),
//                                 rd: A5,
//                                 imm: (-20).into(),
//                                 rs1: S0,
//                                 comment: None,
//                             },
//                             Addiw {
//                                 label: None,
//                                 address: 0x10542.into(),
//                                 rd: A5,
//                                 rs1: A5,
//                                 imm: 5.into(),
//                                 comment: None,
//                             },
//                             Sw {
//                                 label: None,
//                                 address: 0x10544.into(),
//                                 rs2: A5,
//                                 imm: (-20).into(),
//                                 rs1: S0,
//                                 comment: None,
//                             },
//                             Nop {
//                                 label: None,
//                                 address: 0x10548.into(),
//                                 comment: None,
//                             },
//                             J {
//                                 label: None,
//                                 address: 0.into(),
//                                 addr: 0.into(),
//                                 comment: None,
//                             },
//                         ],
//                         continue_target: None,
//                         jump_target: Some(3),
//                     },
//                     BasicBlock {
//                         instructions: vec![Ret {
//                             label: None,
//                             address: 0x1054a.into(),
//                             comment: None,
//                         }],
//                         continue_target: None,
//                         jump_target: None,
//                     },
//                 ],
//                 indirect_targets: vec![
//                     (0x105cc.into(), 3),
//                     (0x105d0.into(), 1),
//                     (0x105e0.into(), 2),
//                 ]
//                 .into_iter()
//                 .collect(),
//             },
//             RiscvFunction {
//                 name: String::from("main"),
//                 basic_blocks: vec![BasicBlock {
//                     instructions: vec![
//                         Jal {
//                             label: Some(String::from("main")),
//                             address: 0x10556.into(),
//                             rd: Ra,
//                             addr: 0x104e0.into(),
//                             comment: Some(String::from("<f>")),
//                         },
//                         Ret {
//                             label: None,
//                             address: 0x1056e.into(),
//                             comment: None,
//                         },
//                     ],
//                     continue_target: None,
//                     jump_target: None,
//                 }],
//                 indirect_targets: HashMap::new(),
//             },
//         ];
//         assert_eq!(cfg, expected);
//     }
// }
