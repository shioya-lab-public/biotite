use crate::riscv_isa::{Abi, Address, CodeBlock, DataBlock, Instruction, Program};
use lazy_static::lazy_static;
use regex::Regex;
use std::mem;

lazy_static! {
    static ref RODATA: Regex = Regex::new(r"Disassembly of section \.rodata:").unwrap();
    static ref CODE: Regex = Regex::new(r"Disassembly of section (\.text.*):").unwrap();
    static ref NOT_DATA: Regex =
        Regex::new(r"Disassembly of section (\.rodata)|(\.comment)|(\.debug.*):").unwrap();
    static ref SECTION: Regex = Regex::new(r"Disassembly of section (.+):").unwrap();
    static ref SYMBOL: Regex = Regex::new(r"([[:xdigit:]]+) <(.*)>:").unwrap();
    static ref BYTES: Regex = Regex::new(r"[[:xdigit:]]+:\s+([[:xdigit:]]+)").unwrap();
}

pub struct Parser<'a> {
    lines: Vec<&'a str>,
    jump_table: Vec<Address>,
    abi: Abi,
    data_blocks: Vec<DataBlock>,
    code_blocks: Vec<CodeBlock>,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Parser {
            lines: Vec::new(),
            jump_table: Vec::new(),
            abi: Abi::default(),
            data_blocks: Vec::new(),
            code_blocks: Vec::new(),
        }
    }

    pub fn run(&mut self, source: &'a str, abi: &Option<String>) -> Program {
        self.lines = source
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .skip_while(|l| !SECTION.is_match(l))
            .collect();
        self.jump_table.clear();
        self.abi = Abi::new(abi);

        assert!(!self.lines.is_empty(), "No disassembly");

        let rodata: Vec<_> = self
            .lines
            .iter()
            .cloned()
            .skip_while(|l| !RODATA.is_match(l))
            .take_while(|l| RODATA.is_match(l) || !SECTION.is_match(l))
            .collect();
        if !rodata.is_empty() {
            let data_blocks = self.parse_rodata_section(&rodata);
            self.data_blocks.extend(data_blocks);
        }

        let mut start = 0;
        let mut end = self.lines[1..]
            .iter()
            .position(|l| SECTION.is_match(l))
            .map(|i| i + 1)
            .unwrap_or(self.lines.len());
        while end <= self.lines.len() {
            let section = &self.lines[start..end];
            if CODE.is_match(section[0]) {
                let code_blocks = self.parse_code_section(section);
                self.code_blocks.extend(code_blocks);
            } else if !NOT_DATA.is_match(section[0]) {
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
                    .map(|i| i + end + 1)
                    .unwrap_or(self.lines.len());
            }
        }

        Program {
            abi: mem::take(&mut self.abi),
            data_blocks: mem::take(&mut self.data_blocks),
            code_blocks: mem::take(&mut self.code_blocks),
        }
    }

    fn parse_rodata_section(&mut self, lines: &[&str]) -> Vec<DataBlock> {
        let data_blocks = self.parse_data_section(lines);
        let addr_lens = match self.abi {
            Abi::Ilp32 | Abi::Ilp32f | Abi::Ilp32d => vec![4],
            Abi::Lp64 | Abi::Lp64f | Abi::Lp64d => vec![4, 8],
        };
        for addr_len in addr_lens {
            for data_block in data_blocks.iter() {
                let mut i = 0;
                while i + addr_len <= data_block.bytes.len() {
                    let mut bytes = data_block.bytes[i..i + addr_len].to_vec();
                    if addr_len == 4 {
                        bytes.extend(vec![0; 4]);
                    }
                    let address = Address(u64::from_le_bytes(bytes.try_into().unwrap()));
                    self.jump_table.push(address);
                    i += 2;
                }
            }
        }
        data_blocks
    }

    fn parse_data_section(&self, lines: &[&str]) -> Vec<DataBlock> {
        let mut data_blocks = Vec::new();
        let section = SECTION.captures(lines[0]).unwrap()[1].to_string();
        let caps = SYMBOL.captures(lines[1]).unwrap();
        let mut symbol = caps[2].to_string();
        let mut address = Address::new(&caps[1]);
        let mut bytes = Vec::new();

        for line in &lines[2..] {
            if let Some(caps) = SYMBOL.captures(line) {
                let symbol = mem::replace(&mut symbol, caps[2].to_string());
                let address = mem::replace(&mut address, Address::new(&caps[1]));
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
                        .to_le_bytes()
                        .to_vec(),
                    8 => u32::from_str_radix(bytes_str, 16)
                        .unwrap()
                        .to_le_bytes()
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
        let mut symbol = caps[2].to_string();
        let mut address = Address::new(&caps[1]);
        let mut instructions = Vec::new();
        let mut split = false;

        for line in &lines[2..] {
            if let Some(caps) = SYMBOL.captures(line) {
                let symbol = mem::replace(&mut symbol, caps[2].to_string());
                let address = mem::replace(&mut address, Address::new(&caps[1]));
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
                    let symbol = mem::replace(&mut symbol, String::new());
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
                | ImplicitJalr { .. }
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

// 323 tests
#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::riscv_isa::{
        Abi::{self, *},
        Address, CodeBlock,
        Csr::*,
        DataBlock,
        FPRegister::*,
        Immediate,
        Instruction::{self, *},
        Iorw,
        Ordering::*,
        Program,
        Register::*,
        Rounding::*,
    };
    use std::env;
    use std::io::Write;
    use std::process::{Command, Stdio};
    use tempfile::NamedTempFile;

    macro_rules! build_tests {
        ( $( $func:ident ( $source:literal,
            $inst:ident { $( $field:ident: $value:expr ),* }
            $(, [$march:literal, $mabi:literal] )?
        ), )* ) => {
            $(
                #[test]
                fn $func() {
                    let source = concat!("
                        main:
                            ", $source,"
                    ");
                    let disasm = compile_and_dump(source, &vec![$($march, $mabi)?]);
                    let inst = Instruction::new(disasm.lines().last().unwrap());
                    assert_eq!(
                        inst,
                        $inst {
                            address: Address(0),
                            $(
                                $field: $value,
                            )*
                        }
                    );
                }
            )*
        };
    }

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
            .args(["-D", "-z", temp_file.path().to_str().unwrap()])
            .output()
            .expect("Unable to invoke `$objdump`");

        String::from_utf8(objdump_proc.stdout).unwrap()
    }

    #[test]
    #[should_panic]
    fn empty_disassembly() {
        let source = compile_and_dump("", &[]);
        Parser::new().run(&source, &None);
    }

    #[test]
    fn abi() {
        let source = compile_and_dump("nop\n", &[]);
        let mut parser = Parser::new();
        let program = parser.run(&source, &None);
        assert_eq!(program.abi, Abi::default());
        let program = parser.run(&source, &Some("ilp32".to_string()));
        assert_eq!(program.abi, Ilp32);
        let program = parser.run(&source, &Some("ilp32f".to_string()));
        assert_eq!(program.abi, Ilp32f);
        let program = parser.run(&source, &Some("ilp32d".to_string()));
        assert_eq!(program.abi, Ilp32d);
        let program = parser.run(&source, &Some("lp64".to_string()));
        assert_eq!(program.abi, Lp64);
        let program = parser.run(&source, &Some("lp64f".to_string()));
        assert_eq!(program.abi, Lp64f);
        let program = parser.run(&source, &Some("lp64d".to_string()));
        assert_eq!(program.abi, Lp64d);
    }

    #[test]
    #[should_panic]
    fn invalid_abi() {
        let source = compile_and_dump("nop\n", &[]);
        Parser::new().run(&source, &Some("invalid".to_string()));
    }

    #[test]
    fn jump_table() {
        let source = "
            .section .rodata
            sym_1:
                .half 1
            sym_2:
                .word 2
            sym_3:
                .word 4
                .word 8
            sym_4:
                .word 16
                .word 32
                .half 64
        ";
        let source = compile_and_dump(source, &[]);

        let mut parser = Parser::new();
        parser.run(&source, &Some("ilp32d".to_string()));
        assert_eq!(
            parser.jump_table,
            vec![
                Address(0x00000002),
                Address(0x00000004),
                Address(0x00080000),
                Address(0x00000008),
                Address(0x00000010),
                Address(0x00200000),
                Address(0x00000020),
                Address(0x00400000),
            ]
        );

        let mut parser = Parser::new();
        parser.run(&source, &Some("lp64d".to_string()));
        assert_eq!(
            parser.jump_table,
            vec![
                Address(0x00000002),
                Address(0x00000004),
                Address(0x00080000),
                Address(0x00000008),
                Address(0x00000010),
                Address(0x00200000),
                Address(0x00000020),
                Address(0x00400000),
                Address(0x00000008_00000004),
                Address(0x00000020_00000010),
                Address(0x00400000_00200000),
            ]
        );
    }

    #[test]
    fn data_section() {
        let source = "
            .section .sdata
                .word 1

            .section .sbss
            sym_1:
                .half 1
            sym_2:
                .word 0x000802b7 # lui t0,128

            .section .comment
                .word 1

            .section .debug_info
                .word 1
        ";
        let source = compile_and_dump(source, &[]);
        let program = Parser::new().run(&source, &None);
        assert_eq!(
            program,
            Program {
                abi: Abi::default(),
                code_blocks: Vec::new(),
                data_blocks: vec![
                    DataBlock {
                        section: String::from(".sdata"),
                        symbol: String::from(".sdata"),
                        address: Address(0x0),
                        bytes: vec![0x1, 0x0, 0x0, 0x0],
                    },
                    DataBlock {
                        section: String::from(".sbss"),
                        symbol: String::from("sym_1"),
                        address: Address(0x0),
                        bytes: vec![0x1, 0x0],
                    },
                    DataBlock {
                        section: String::from(".sbss"),
                        symbol: String::from("sym_2"),
                        address: Address(0x2),
                        bytes: vec![0xb7, 0x02, 0x08, 0x0],
                    }
                ],
            }
        );
    }

    #[test]
    fn code_section() {
        let source = "
            .section .text
            sym_1:
                lui t0,0
            sym_2:
                lui t0,128

            .section .text.startup
            main:
                jal ra,main
                jalr t0,4(t0)
                jalr ra,4(t0)
                beq t0,t1,main
                bne t0,t1,main
                blt t0,t1,main
                bge t0,t1,main
                bltu t0,t1,main
                bgeu t0,t1,main
                beqz t0,main
                bnez t0,main
                blez t0,main
                bgez t0,main
                bltz t0,main
                bgtz t0,main
                j main
                jr t0
                jalr t0
                ret
        ";
        let source = compile_and_dump(source, &[]);
        let program = Parser::new().run(&source, &None);
        assert_eq!(
            program,
            Program {
                abi: Abi::default(),
                code_blocks: vec![
                    CodeBlock {
                        section: String::from(".text"),
                        symbol: String::from("sym_1"),
                        address: Address(0x0),
                        instructions: vec![Lui {
                            address: Address(0x0),
                            rd: T0,
                            imm: Immediate(0),
                        }],
                    },
                    CodeBlock {
                        section: String::from(".text"),
                        symbol: String::from("sym_2"),
                        address: Address(0x4),
                        instructions: vec![Lui {
                            address: Address(0x4),
                            rd: T0,
                            imm: Immediate(128),
                        }],
                    },
                    CodeBlock {
                        section: String::from(".text.startup"),
                        symbol: String::from("main"),
                        address: Address(0x0),
                        instructions: vec![Jal {
                            address: Address(0x0),
                            rd: Ra,
                            addr: Address(0x0),
                        }],
                    },
                    CodeBlock {
                        section: String::from(".text.startup"),
                        symbol: String::new(),
                        address: Address(0x4),
                        instructions: vec![Jalr {
                            address: Address(0x4),
                            rd: T0,
                            imm: Immediate(4),
                            rs1: T0,
                        }],
                    },
                    CodeBlock {
                        section: String::from(".text.startup"),
                        symbol: String::new(),
                        address: Address(0x8),
                        instructions: vec![ImplicitJalr {
                            address: Address(0x8),
                            imm: Immediate(4),
                            rs1: T0,
                        }],
                    },
                    CodeBlock {
                        section: String::from(".text.startup"),
                        symbol: String::new(),
                        address: Address(0xc),
                        instructions: vec![Beq {
                            address: Address(0xc),
                            rs1: T0,
                            rs2: T1,
                            addr: Address(0x0),
                        }],
                    },
                    CodeBlock {
                        section: String::from(".text.startup"),
                        symbol: String::new(),
                        address: Address(0x10),
                        instructions: vec![Bne {
                            address: Address(0x10),
                            rs1: T0,
                            rs2: T1,
                            addr: Address(0x0),
                        }],
                    },
                    CodeBlock {
                        section: String::from(".text.startup"),
                        symbol: String::new(),
                        address: Address(0x14),
                        instructions: vec![Blt {
                            address: Address(0x14),
                            rs1: T0,
                            rs2: T1,
                            addr: Address(0x0),
                        }],
                    },
                    CodeBlock {
                        section: String::from(".text.startup"),
                        symbol: String::new(),
                        address: Address(0x18),
                        instructions: vec![Bge {
                            address: Address(0x18),
                            rs1: T0,
                            rs2: T1,
                            addr: Address(0x0),
                        }],
                    },
                    CodeBlock {
                        section: String::from(".text.startup"),
                        symbol: String::new(),
                        address: Address(0x1c),
                        instructions: vec![Bltu {
                            address: Address(0x1c),
                            rs1: T0,
                            rs2: T1,
                            addr: Address(0x0),
                        }],
                    },
                    CodeBlock {
                        section: String::from(".text.startup"),
                        symbol: String::new(),
                        address: Address(0x20),
                        instructions: vec![Bgeu {
                            address: Address(0x20),
                            rs1: T0,
                            rs2: T1,
                            addr: Address(0x0),
                        }],
                    },
                    CodeBlock {
                        section: String::from(".text.startup"),
                        symbol: String::new(),
                        address: Address(0x24),
                        instructions: vec![Beqz {
                            address: Address(0x24),
                            rs1: T0,
                            addr: Address(0x0),
                        }],
                    },
                    CodeBlock {
                        section: String::from(".text.startup"),
                        symbol: String::new(),
                        address: Address(0x28),
                        instructions: vec![Bnez {
                            address: Address(0x28),
                            rs1: T0,
                            addr: Address(0x0),
                        }],
                    },
                    CodeBlock {
                        section: String::from(".text.startup"),
                        symbol: String::new(),
                        address: Address(0x2c),
                        instructions: vec![Blez {
                            address: Address(0x2c),
                            rs1: T0,
                            addr: Address(0x0),
                        }],
                    },
                    CodeBlock {
                        section: String::from(".text.startup"),
                        symbol: String::new(),
                        address: Address(0x30),
                        instructions: vec![Bgez {
                            address: Address(0x30),
                            rs1: T0,
                            addr: Address(0x0),
                        }],
                    },
                    CodeBlock {
                        section: String::from(".text.startup"),
                        symbol: String::new(),
                        address: Address(0x34),
                        instructions: vec![Bltz {
                            address: Address(0x34),
                            rs1: T0,
                            addr: Address(0x0),
                        }],
                    },
                    CodeBlock {
                        section: String::from(".text.startup"),
                        symbol: String::new(),
                        address: Address(0x38),
                        instructions: vec![Bgtz {
                            address: Address(0x38),
                            rs1: T0,
                            addr: Address(0x0),
                        }],
                    },
                    CodeBlock {
                        section: String::from(".text.startup"),
                        symbol: String::new(),
                        address: Address(0x3c),
                        instructions: vec![J {
                            address: Address(0x3c),
                            addr: Address(0x0),
                        }],
                    },
                    CodeBlock {
                        section: String::from(".text.startup"),
                        symbol: String::new(),
                        address: Address(0x3e),
                        instructions: vec![Jr {
                            address: Address(0x3e),
                            rs1: T0,
                        }],
                    },
                    CodeBlock {
                        section: String::from(".text.startup"),
                        symbol: String::new(),
                        address: Address(0x40),
                        instructions: vec![PseudoJalr {
                            address: Address(0x40),
                            rs1: T0,
                        }],
                    },
                    CodeBlock {
                        section: String::from(".text.startup"),
                        symbol: String::new(),
                        address: Address(0x42),
                        instructions: vec![Ret {
                            address: Address(0x42),
                        }],
                    },
                ],
                data_blocks: Vec::new(),
            }
        );
    }

    build_tests! {
        // Registers (32 tests)
        reg_1("flw ft0,-4(zero)", Flw { frd: Ft0, imm: Immediate(-4), rs1: Zero }),
        reg_2("flw ft1,-4(ra)", Flw { frd: Ft1, imm: Immediate(-4), rs1: Ra }),
        reg_3("flw ft2,-4(sp)", Flw { frd: Ft2, imm: Immediate(-4), rs1: Sp }),
        reg_4("flw ft3,-4(gp)", Flw { frd: Ft3, imm: Immediate(-4), rs1: Gp }),
        reg_5("flw ft4,-4(tp)", Flw { frd: Ft4, imm: Immediate(-4), rs1: Tp }),
        reg_6("flw ft5,-4(t0)", Flw { frd: Ft5, imm: Immediate(-4), rs1: T0 }),
        reg_7("flw ft6,-4(t1)", Flw { frd: Ft6, imm: Immediate(-4), rs1: T1 }),
        reg_8("flw ft7,-4(t2)", Flw { frd: Ft7, imm: Immediate(-4), rs1: T2 }),
        reg_9("flw fs0,-4(s0)", Flw { frd: Fs0, imm: Immediate(-4), rs1: S0 }),
        reg_10("flw fs1,-4(s1)", Flw { frd: Fs1, imm: Immediate(-4), rs1: S1 }),
        reg_11("flw fa0,-4(a0)", Flw { frd: Fa0, imm: Immediate(-4), rs1: A0 }),
        reg_12("flw fa1,-4(a1)", Flw { frd: Fa1, imm: Immediate(-4), rs1: A1 }),
        reg_13("flw fa2,-4(a2)", Flw { frd: Fa2, imm: Immediate(-4), rs1: A2 }),
        reg_14("flw fa3,-4(a3)", Flw { frd: Fa3, imm: Immediate(-4), rs1: A3 }),
        reg_15("flw fa4,-4(a4)", Flw { frd: Fa4, imm: Immediate(-4), rs1: A4 }),
        reg_16("flw fa5,-4(a5)", Flw { frd: Fa5, imm: Immediate(-4), rs1: A5 }),
        reg_17("flw fa6,-4(a6)", Flw { frd: Fa6, imm: Immediate(-4), rs1: A6 }),
        reg_18("flw fa7,-4(a7)", Flw { frd: Fa7, imm: Immediate(-4), rs1: A7 }),
        reg_19("flw fs2,-4(s2)", Flw { frd: Fs2, imm: Immediate(-4), rs1: S2 }),
        reg_20("flw fs3,-4(s3)", Flw { frd: Fs3, imm: Immediate(-4), rs1: S3 }),
        reg_21("flw fs4,-4(s4)", Flw { frd: Fs4, imm: Immediate(-4), rs1: S4 }),
        reg_22("flw fs5,-4(s5)", Flw { frd: Fs5, imm: Immediate(-4), rs1: S5 }),
        reg_23("flw fs6,-4(s6)", Flw { frd: Fs6, imm: Immediate(-4), rs1: S6 }),
        reg_24("flw fs7,-4(s7)", Flw { frd: Fs7, imm: Immediate(-4), rs1: S7 }),
        reg_25("flw fs8,-4(s8)", Flw { frd: Fs8, imm: Immediate(-4), rs1: S8 }),
        reg_26("flw fs9,-4(s9)", Flw { frd: Fs9, imm: Immediate(-4), rs1: S9 }),
        reg_27("flw fs10,-4(s10)", Flw { frd: Fs10, imm: Immediate(-4), rs1: S10 }),
        reg_28("flw fs11,-4(s11)", Flw { frd: Fs11, imm: Immediate(-4), rs1: S11 }),
        reg_29("flw ft8,-4(t3)", Flw { frd: Ft8, imm: Immediate(-4), rs1: T3 }),
        reg_30("flw ft9,-4(t4)", Flw { frd: Ft9, imm: Immediate(-4), rs1: T4 }),
        reg_31("flw ft10,-4(t5)", Flw { frd: Ft10, imm: Immediate(-4), rs1: T5 }),
        reg_32("flw ft11,-4(t6)", Flw { frd: Ft11, imm: Immediate(-4), rs1: T6 }),

        // RV32I (42 tests)
        lui("lui t0,4", Lui { rd: T0, imm: Immediate(4) }),
        auipc("auipc t0,4", Auipc { rd: T0, imm: Immediate(4) }),
        jal("jal ra,main", Jal { rd: Ra, addr: Address(0x0) }),
        jalr("jalr t0,4(t0)", Jalr { rd: T0, imm: Immediate(4), rs1: T0 }),
        implicit_jalr("jalr 4(t0)", ImplicitJalr { imm: Immediate(4), rs1: T0 }),
        beq("beq t0,t1,main", Beq { rs1: T0, rs2: T1, addr: Address(0x0) }),
        bne("bne t0,t1,main", Bne { rs1: T0, rs2: T1, addr: Address(0x0) }),
        blt("blt t0,t1,main", Blt { rs1: T0, rs2: T1, addr: Address(0x0) }),
        bge("bge t0,t1,main", Bge { rs1: T0, rs2: T1, addr: Address(0x0) }),
        bltu("bltu t0,t1,main", Bltu { rs1: T0, rs2: T1, addr: Address(0x0) }),
        bgeu("bgeu t0,t1,main", Bgeu { rs1: T0, rs2: T1, addr: Address(0x0) }),
        lb("lb t0,-4(t1)", Lb { rd: T0, imm: Immediate(-4), rs1: T1 }),
        lh("lh t0,-4(t1)", Lh { rd: T0, imm: Immediate(-4), rs1: T1 }),
        lw("lw t0,-4(t1)", Lw { rd: T0, imm: Immediate(-4), rs1: T1 }),
        lbu("lbu t0,-4(t1)", Lbu { rd: T0, imm: Immediate(-4), rs1: T1 }),
        lhu("lhu t0,-4(t1)", Lhu { rd: T0, imm: Immediate(-4), rs1: T1 }),
        sb("sb t0,-4(t1)", Sb { rs2: T0, imm: Immediate(-4), rs1: T1 }),
        sh("sh t0,-4(t1)", Sh { rs2: T0, imm: Immediate(-4), rs1: T1 }),
        sw("sw t0,-4(t1)", Sw { rs2: T0, imm: Immediate(-4), rs1: T1 }),
        addi("addi t0,t1,4", Addi { rd: T0, rs1: T1, imm: Immediate(4) }),
        slti("slti t0,t1,4", Slti { rd: T0, rs1: T1, imm: Immediate(4) }),
        sltiu("sltiu t0,t1,4", Sltiu { rd: T0, rs1: T1, imm: Immediate(4) }),
        xori("xori t0,t1,4", Xori { rd: T0, rs1: T1, imm:Immediate(4) }),
        ori("ori t0,t1,4", Ori { rd: T0, rs1: T1, imm: Immediate(4) }),
        andi("andi t0,t1,4", Andi { rd: T0, rs1: T1, imm: Immediate(4) }),
        slli("slli t0,t1,4", Slli { rd: T0, rs1: T1, imm: Immediate(4) }),
        srli("srli t0,t1,4", Srli { rd: T0, rs1: T1, imm: Immediate(4) }),
        srai("srai t0,t1,4", Srai { rd: T0, rs1: T1, imm: Immediate(4) }),
        add("add t0,t1,t2", Add { rd: T0, rs1: T1, rs2: T2 }),
        sub("sub t0,t1,t2", Sub { rd: T0, rs1: T1, rs2: T2 }),
        sll("sll t0,t1,t2", Sll { rd: T0, rs1: T1, rs2: T2 }),
        slt("slt t0,t1,t2", Slt { rd: T0, rs1: T1, rs2: T2 }),
        sltu("sltu t0,t1,t2", Sltu { rd: T0, rs1: T1, rs2: T2 }),
        xor("xor t0,t1,t2", Xor { rd: T0, rs1: T1, rs2: T2 }),
        srl("srl t0,t1,t2", Srl { rd: T0, rs1: T1, rs2: T2 }),
        sra("sra t0,t1,t2", Sra { rd: T0, rs1: T1, rs2: T2 }),
        or("or t0,t1,t2", Or { rd: T0, rs1: T1, rs2: T2 }),
        and("and t0,t1,t2", And { rd: T0, rs1: T1, rs2: T2 }),
        fence_tso("fence.tso", Fence { iorw: Iorw("tso".to_string()) }),
        fence("fence io,rw", Fence { iorw: Iorw("io,rw".to_string()) }),
        ecall("ecall", Ecall {}),
        ebreak("ebreak", Ebreak {}),

        // RV64I (12 tests)
        lwu("lwu t0,-4(t1)", Lwu { rd: T0, imm: Immediate(-4), rs1: T1 }),
        ld("ld t0,-4(t1)", Ld { rd: T0, imm: Immediate(-4), rs1: T1 }),
        sd("sd t0,-4(t1)", Sd { rs2: T0, imm: Immediate(-4), rs1: T1 }),
        addiw("addiw t0,t1,4", Addiw { rd: T0, rs1: T1, imm: Immediate(4) }),
        slliw("slliw t0,t1,4", Slliw { rd: T0, rs1: T1, imm: Immediate(4) }),
        srliw("srliw t0,t1,4", Srliw { rd: T0, rs1: T1, imm: Immediate(4) }),
        sraiw("sraiw t0,t1,4", Sraiw { rd: T0, rs1: T1, imm: Immediate(4) }),
        addw("addw t0,t1,t2", Addw { rd: T0, rs1: T1, rs2: T2 }),
        subw("subw t0,t1,t2", Subw { rd: T0, rs1: T1, rs2: T2 }),
        sllw("sllw t0,t1,t2", Sllw { rd: T0, rs1: T1, rs2: T2 }),
        srlw("srlw t0,t1,t2", Srlw { rd: T0, rs1: T1, rs2: T2 }),
        sraw("sraw t0,t1,t2", Sraw { rd: T0, rs1: T1, rs2: T2 }),

        // RV32/RV64 Zifencei (1 test)
        fence_i("fence.i", FenceI {}),

        // RV32/RV64 Zicsr (10 tests)
        csrrw("csrrw t0,cycle,t1", Csrrw { rd: T0, csr: Cycle, rs1: T1 }),
        csrrs("csrrs t0,time,t1", Csrrs { rd: T0, csr: Time, rs1: T1 }),
        csrrc("csrrc t0,instret,t1", Csrrc { rd: T0, csr: Instret, rs1: T1 }),
        csrrwi("csrrwi t0,cycleh,4", Csrrwi { rd: T0, csr: Cycleh, imm: Immediate(4) }, ["-march=rv32gc", "-mabi=ilp32d"]),
        csrrsi("csrrsi t0,timeh,4", Csrrsi {rd: T0, csr: Timeh, imm: Immediate(4) }, ["-march=rv32gc", "-mabi=ilp32d"]),
        csrrci("csrrci t0,instreth,4", Csrrci {rd: T0, csr: Instreth, imm: Immediate(4) }, ["-march=rv32gc", "-mabi=ilp32d"]),
        csr_fflags("csrrc t0,fflags,t1", Csrrc { rd: T0, csr: Fflags, rs1: T1 }),
        csr_frm("csrrc t0,frm,t1", Csrrc { rd: T0, csr: Frm, rs1: T1 }),
        csr_fcsr("csrrc t0,fcsr,t1", Csrrc { rd: T0, csr: Fcsr, rs1: T1 }),
        csr_unknown("csrrc t0,0x99,t1", Csrrc { rd: T0, csr: Unknown(0x99), rs1: T1 }),

        // RV32M (8 tests)
        mul("mul t0,t1,t2", Mul { rd: T0, rs1: T1, rs2: T2 }),
        mulh("mulh t0,t1,t2", Mulh { rd: T0, rs1: T1, rs2: T2 }),
        mulhsu("mulhsu t0,t1,t2", Mulhsu { rd: T0, rs1: T1, rs2: T2 }),
        mulhu("mulhu t0,t1,t2", Mulhu { rd: T0, rs1: T1, rs2: T2 }),
        div("div t0,t1,t2", Div { rd: T0, rs1: T1, rs2: T2 }),
        divu("divu t0,t1,t2", Divu { rd: T0, rs1: T1, rs2: T2 }),
        rem("rem t0,t1,t2", Rem { rd: T0, rs1: T1, rs2: T2 }),
        remu("remu t0,t1,t2", Remu { rd: T0, rs1: T1, rs2: T2 }),

        // RV64M (5 tests)
        mulw("mulw t0,t1,t2", Mulw { rd: T0, rs1: T1, rs2: T2 }),
        divw("divw t0,t1,t2", Divw { rd: T0, rs1: T1, rs2: T2 }),
        divuw("divuw t0,t1,t2", Divuw { rd: T0, rs1: T1, rs2: T2 }),
        remw("remw t0,t1,t2", Remw { rd: T0, rs1: T1, rs2: T2 }),
        remuw("remuw t0,t1,t2", Remuw { rd: T0, rs1: T1, rs2: T2 }),

        // RV32A (44 tests)
        lr_w("lr.w t0,(t1)", LrW { ord: No, rd: T0, rs1: T1 }),
        lr_w_aq("lr.w.aq t0,(t1)", LrW { ord: Aq, rd: T0, rs1: T1 }),
        lr_w_rl("lr.w.rl t0,(t1)", LrW { ord: Rl, rd: T0, rs1: T1 }),
        lr_w_aqrl("lr.w.aqrl t0,(t1)", LrW { ord: AqRl, rd: T0, rs1: T1 }),
        sc_w("sc.w t0,t1,(t2)", ScW { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        sc_w_aq("sc.w.aq t0,t1,(t2)", ScW { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        sc_w_rl("sc.w.rl t0,t1,(t2)", ScW { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        sc_w_aqrl("sc.w.aqrl t0,t1,(t2)", ScW { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        amoswap_w("amoswap.w t0,t1,(t2)", AmoswapW { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        amoswap_w_aq("amoswap.w.aq t0,t1,(t2)", AmoswapW { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        amoswap_w_rl("amoswap.w.rl t0,t1,(t2)", AmoswapW { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        amoswap_w_aqrl("amoswap.w.aqrl t0,t1,(t2)", AmoswapW { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        amoadd_w("amoadd.w t0,t1,(t2)", AmoaddW { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        amoadd_w_aq("amoadd.w.aq t0,t1,(t2)", AmoaddW { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        amoadd_w_rl("amoadd.w.rl t0,t1,(t2)", AmoaddW { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        amoadd_w_aqrl("amoadd.w.aqrl t0,t1,(t2)", AmoaddW { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        amoxor_w("amoxor.w t0,t1,(t2)", AmoxorW { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        amoxor_w_aq("amoxor.w.aq t0,t1,(t2)", AmoxorW { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        amoxor_w_rl("amoxor.w.rl t0,t1,(t2)", AmoxorW { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        amoxor_w_aqrl("amoxor.w.aqrl t0,t1,(t2)", AmoxorW { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        amoand_w("amoand.w t0,t1,(t2)", AmoandW { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        amoand_w_aq("amoand.w.aq t0,t1,(t2)", AmoandW { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        amoand_w_rl("amoand.w.rl t0,t1,(t2)", AmoandW { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        amoand_w_aqrl("amoand.w.aqrl t0,t1,(t2)", AmoandW { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        amoor_w("amoor.w t0,t1,(t2)", AmoorW { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        amoor_w_aq("amoor.w.aq t0,t1,(t2)", AmoorW { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        amoor_w_rl("amoor.w.rl t0,t1,(t2)", AmoorW { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        amoor_w_aqrl("amoor.w.aqrl t0,t1,(t2)", AmoorW { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        amomin_w("amomin.w t0,t1,(t2)", AmominW { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        amomin_w_aq("amomin.w.aq t0,t1,(t2)", AmominW { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        amomin_w_rl("amomin.w.rl t0,t1,(t2)", AmominW { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        amomin_w_aqrl("amomin.w.aqrl t0,t1,(t2)", AmominW { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        amomax_w("amomax.w t0,t1,(t2)", AmomaxW { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        amomax_w_aq("amomax.w.aq t0,t1,(t2)", AmomaxW { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        amomax_w_rl("amomax.w.rl t0,t1,(t2)", AmomaxW { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        amomax_w_aqrl("amomax.w.aqrl t0,t1,(t2)", AmomaxW { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        amominu_w("amominu.w t0,t1,(t2)", AmominuW { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        amominu_w_aq("amominu.w.aq t0,t1,(t2)", AmominuW { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        amominu_w_rl("amominu.w.rl t0,t1,(t2)", AmominuW { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        amominu_w_aqrl("amominu.w.aqrl t0,t1,(t2)", AmominuW { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        amomaxu_w("amomaxu.w t0,t1,(t2)", AmomaxuW { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        amomaxu_w_aq("amomaxu.w.aq t0,t1,(t2)", AmomaxuW { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        amomaxu_w_rl("amomaxu.w.rl t0,t1,(t2)", AmomaxuW { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        amomaxu_w_aqrl("amomaxu.w.aqrl t0,t1,(t2)", AmomaxuW { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),

        // RV64A (44 tests)
        lr_d("lr.d t0,(t1)", LrD { ord: No, rd: T0, rs1: T1 }),
        lr_d_aq("lr.d.aq t0,(t1)", LrD { ord: Aq, rd: T0, rs1: T1 }),
        lr_d_rl("lr.d.rl t0,(t1)", LrD { ord: Rl, rd: T0, rs1: T1 }),
        lr_d_aqrl("lr.d.aqrl t0,(t1)", LrD { ord: AqRl, rd: T0, rs1: T1 }),
        sc_d("sc.d t0,t1,(t2)", ScD { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        sc_d_aq("sc.d.aq t0,t1,(t2)", ScD { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        sc_d_rl("sc.d.rl t0,t1,(t2)", ScD { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        sc_d_aqrl("sc.d.aqrl t0,t1,(t2)", ScD { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        amoswap_d("amoswap.d t0,t1,(t2)", AmoswapD { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        amoswap_d_aq("amoswap.d.aq t0,t1,(t2)", AmoswapD { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        amoswap_d_rl("amoswap.d.rl t0,t1,(t2)", AmoswapD { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        amoswap_d_aqrl("amoswap.d.aqrl t0,t1,(t2)", AmoswapD { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        amoadd_d("amoadd.d t0,t1,(t2)", AmoaddD { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        amoadd_d_aq("amoadd.d.aq t0,t1,(t2)", AmoaddD { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        amoadd_d_rl("amoadd.d.rl t0,t1,(t2)", AmoaddD { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        amoadd_d_aqrl("amoadd.d.aqrl t0,t1,(t2)", AmoaddD { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        amoxor_d("amoxor.d t0,t1,(t2)", AmoxorD { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        amoxor_d_aq("amoxor.d.aq t0,t1,(t2)", AmoxorD { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        amoxor_d_rl("amoxor.d.rl t0,t1,(t2)", AmoxorD { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        amoxor_d_aqrl("amoxor.d.aqrl t0,t1,(t2)", AmoxorD { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        amoand_d("amoand.d t0,t1,(t2)", AmoandD { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        amoand_d_aq("amoand.d.aq t0,t1,(t2)", AmoandD { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        amoand_d_rl("amoand.d.rl t0,t1,(t2)", AmoandD { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        amoand_d_aqrl("amoand.d.aqrl t0,t1,(t2)", AmoandD { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        amoor_d("amoor.d t0,t1,(t2)", AmoorD { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        amoor_d_aq("amoor.d.aq t0,t1,(t2)", AmoorD { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        amoor_d_rl("amoor.d.rl t0,t1,(t2)", AmoorD { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        amoor_d_aqrl("amoor.d.aqrl t0,t1,(t2)", AmoorD { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        amomin_d("amomin.d t0,t1,(t2)", AmominD { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        amomin_d_aq("amomin.d.aq t0,t1,(t2)", AmominD { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        amomin_d_rl("amomin.d.rl t0,t1,(t2)", AmominD { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        amomin_d_aqrl("amomin.d.aqrl t0,t1,(t2)", AmominD { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        amomax_d("amomax.d t0,t1,(t2)", AmomaxD { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        amomax_d_aq("amomax.d.aq t0,t1,(t2)", AmomaxD { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        amomax_d_rl("amomax.d.rl t0,t1,(t2)", AmomaxD { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        amomax_d_aqrl("amomax.d.aqrl t0,t1,(t2)", AmomaxD { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        amominu_d("amominu.d t0,t1,(t2)", AmominuD { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        amominu_d_aq("amominu.d.aq t0,t1,(t2)", AmominuD { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        amominu_d_rl("amominu.d.rl t0,t1,(t2)", AmominuD { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        amominu_d_aqrl("amominu.d.aqrl t0,t1,(t2)", AmominuD { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),
        amomaxu_d("amomaxu.d t0,t1,(t2)", AmomaxuD { ord: No, rd: T0, rs2: T1, rs1: T2 }),
        amomaxu_d_aq("amomaxu.d.aq t0,t1,(t2)", AmomaxuD { ord: Aq, rd: T0, rs2: T1, rs1: T2 }),
        amomaxu_d_rl("amomaxu.d.rl t0,t1,(t2)", AmomaxuD { ord: Rl, rd: T0, rs2: T1, rs1: T2 }),
        amomaxu_d_aqrl("amomaxu.d.aqrl t0,t1,(t2)", AmomaxuD { ord: AqRl, rd: T0, rs2: T1, rs1: T2 }),

        // RV32F (33 tests)
        rm_rne("fmadd.s ft0,ft1,ft2,ft3,rne", FmaddS { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Rne }),
        rm_rtz("fmadd.s ft0,ft1,ft2,ft3,rtz", FmaddS { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Rtz }),
        rm_rdn("fmadd.s ft0,ft1,ft2,ft3,rdn", FmaddS { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Rdn }),
        rm_rup("fmadd.s ft0,ft1,ft2,ft3,rup", FmaddS { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Rup }),
        rm_rmm("fmadd.s ft0,ft1,ft2,ft3,rmm", FmaddS { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Rmm }),
        rm_dyn("fmadd.s ft0,ft1,ft2,ft3,dyn", FmaddS { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Dyn }),
        rm_default("fmadd.s ft0,ft1,ft2,ft3", FmaddS { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Dyn }),

        flw("flw ft0,-4(t0)", Flw { frd: Ft0, imm: Immediate(-4), rs1: T0 }),
        fsw("fsw ft0,-4(t0)", Fsw { frs2: Ft0, imm: Immediate(-4), rs1: T0 }),
        fmadd_s("fmadd.s ft0,ft1,ft2,ft3", FmaddS { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Dyn }),
        fmsub_s("fmsub.s ft0,ft1,ft2,ft3", FmsubS { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Dyn }),
        fnmsub_s("fnmsub.s ft0,ft1,ft2,ft3", FnmsubS { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Dyn }),
        fnmadd_s("fnmadd.s ft0,ft1,ft2,ft3", FnmaddS { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Dyn }),
        fadd_s("fadd.s ft0,ft1,ft2", FaddS { frd: Ft0, frs1: Ft1, frs2: Ft2, rm: Dyn }),
        fsub_s("fsub.s ft0,ft1,ft2", FsubS { frd: Ft0, frs1: Ft1, frs2: Ft2, rm: Dyn }),
        fmul_s("fmul.s ft0,ft1,ft2", FmulS { frd: Ft0, frs1: Ft1, frs2: Ft2, rm: Dyn }),
        fdiv_s("fdiv.s ft0,ft1,ft2", FdivS { frd: Ft0, frs1: Ft1, frs2: Ft2, rm: Dyn }),
        fsqrt_s("fsqrt.s ft0,ft1", FsqrtS { frd: Ft0, frs1: Ft1, rm: Dyn }),
        fsgnj_s("fsgnj.s ft0,ft1,ft2", FsgnjS { frd: Ft0, frs1: Ft1, frs2: Ft2 }),
        fsgnjn_s("fsgnjn.s ft0,ft1,ft2", FsgnjnS { frd: Ft0, frs1: Ft1, frs2: Ft2 }),
        fsgnjx_s("fsgnjx.s ft0,ft1,ft2", FsgnjxS { frd: Ft0, frs1: Ft1, frs2: Ft2 }),
        fmin_s("fmin.s ft0,ft1,ft2", FminS { frd: Ft0, frs1: Ft1, frs2: Ft2 }),
        fmax_s("fmax.s ft0,ft1,ft2", FmaxS { frd: Ft0, frs1: Ft1, frs2: Ft2 }),
        fcvt_w_s("fcvt.w.s t0,ft0", FcvtWS { rd: T0, frs1: Ft0, rm: Dyn }),
        fcvt_wu_s("fcvt.wu.s t0,ft0", FcvtWuS { rd: T0, frs1: Ft0, rm: Dyn }),
        fmv_x_w("fmv.x.w t0,ft0", FmvXW { rd: T0, frs1: Ft0 }),
        feq_s("feq.s t0,ft0,ft1", FeqS { rd: T0, frs1: Ft0, frs2: Ft1 }),
        flt_s("flt.s t0,ft0,ft1", FltS { rd: T0, frs1: Ft0, frs2: Ft1 }),
        fle_s("fle.s t0,ft0,ft1", FleS { rd: T0, frs1: Ft0, frs2: Ft1 }),
        fclass_s("fclass.s t0,ft0", FclassS { rd: T0, frs1: Ft0 }),
        fcvt_s_w("fcvt.s.w ft0,t0", FcvtSW { frd: Ft0, rs1: T0, rm: Dyn }),
        fcvt_s_wu("fcvt.s.wu ft0,t0", FcvtSWu { frd: Ft0, rs1: T0, rm: Dyn }),
        fmv_w_x("fmv.w.x ft0,t0", FmvWX { frd: Ft0, rs1: T0 }),

        // RV64F (4 tests)
        fcvt_l_s("fcvt.l.s t0,ft0", FcvtLS { rd: T0, frs1: Ft0, rm: Dyn }),
        fcvt_lu_s("fcvt.lu.s t0,ft0", FcvtLuS { rd: T0, frs1: Ft0, rm: Dyn }),
        fcvt_s_l("fcvt.s.l ft0,t0", FcvtSL { frd: Ft0, rs1: T0, rm: Dyn }),
        fcvt_s_lu("fcvt.s.lu ft0,t0", FcvtSLu { frd: Ft0, rs1: T0, rm: Dyn }),

        // RV32D (26 tests)
        fld("fld ft0,-4(t0)", Fld { frd: Ft0, imm: Immediate(-4), rs1: T0 }),
        fsd("fsd ft0,-4(t0)", Fsd { frs2: Ft0, imm: Immediate(-4), rs1: T0 }),
        fmadd_d("fmadd.d ft0,ft1,ft2,ft3", FmaddD { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Dyn }),
        fmsub_d("fmsub.d ft0,ft1,ft2,ft3", FmsubD { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Dyn }),
        fnmsub_d("fnmsub.d ft0,ft1,ft2,ft3", FnmsubD { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Dyn }),
        fnmadd_d("fnmadd.d ft0,ft1,ft2,ft3", FnmaddD { frd: Ft0, frs1: Ft1, frs2: Ft2, frs3: Ft3, rm: Dyn }),
        fadd_d("fadd.d ft0,ft1,ft2", FaddD { frd: Ft0, frs1: Ft1, frs2: Ft2, rm: Dyn }),
        fsub_d("fsub.d ft0,ft1,ft2", FsubD { frd: Ft0, frs1: Ft1, frs2: Ft2, rm: Dyn }),
        fmul_d("fmul.d ft0,ft1,ft2", FmulD { frd: Ft0, frs1: Ft1, frs2: Ft2, rm: Dyn }),
        fdiv_d("fdiv.d ft0,ft1,ft2", FdivD { frd: Ft0, frs1: Ft1, frs2: Ft2, rm: Dyn }),
        fsqrt_d("fsqrt.d ft0,ft1", FsqrtD { frd: Ft0, frs1: Ft1, rm: Dyn }),
        fsgnj_d("fsgnj.d ft0,ft1,ft2", FsgnjD { frd: Ft0, frs1: Ft1, frs2: Ft2 }),
        fsgnjn_d("fsgnjn.d ft0,ft1,ft2", FsgnjnD { frd: Ft0, frs1: Ft1, frs2: Ft2 }),
        fsgnjx_d("fsgnjx.d ft0,ft1,ft2", FsgnjxD { frd: Ft0, frs1: Ft1, frs2: Ft2 }),
        fmin_d("fmin.d ft0,ft1,ft2", FminD { frd: Ft0, frs1: Ft1, frs2: Ft2 }),
        fmax_d("fmax.d ft0,ft1,ft2", FmaxD { frd: Ft0, frs1: Ft1, frs2: Ft2 }),
        fcvt_s_d("fcvt.s.d ft0,ft1", FcvtSD { frd: Ft0, frs1: Ft1, rm: Dyn }),
        fcvt_d_s("fcvt.d.s ft0,ft1", FcvtDS { frd: Ft0, frs1: Ft1, rm: Dyn }),
        feq_d("feq.d t0,ft0,ft1", FeqD { rd: T0, frs1: Ft0, frs2: Ft1 }),
        flt_d("flt.d t0,ft0,ft1", FltD { rd: T0, frs1: Ft0, frs2: Ft1 }),
        fle_d("fle.d t0,ft0,ft1", FleD { rd: T0, frs1: Ft0, frs2: Ft1 }),
        fclass_d("fclass.d t0,ft0", FclassD { rd: T0, frs1: Ft0 }),
        fcvt_w_d("fcvt.w.d t0,ft0", FcvtWD { rd: T0, frs1: Ft0, rm: Dyn }),
        fcvt_wu_d("fcvt.wu.d t0,ft0", FcvtWuD { rd: T0, frs1: Ft0, rm: Dyn }),
        fcvt_d_w("fcvt.d.w ft0,t0", FcvtDW { frd: Ft0, rs1: T0, rm: Dyn }),
        fcvt_d_wu("fcvt.d.wu ft0,t0", FcvtDWu { frd: Ft0, rs1: T0, rm: Dyn }),

        // RV64D (6 tests)
        fcvt_l_d("fcvt.l.d t0,ft0", FcvtLD { rd: T0, frs1: Ft0, rm: Dyn }),
        fcvt_lu_d("fcvt.lu.d t0,ft0", FcvtLuD { rd: T0, frs1: Ft0, rm: Dyn }),
        fmv_x_d("fmv.x.d t0,ft0", FmvXD { rd: T0, frs1: Ft0 }),
        fcvt_d_l("fcvt.d.l ft0,t0", FcvtDL { frd: Ft0, rs1: T0, rm: Dyn }),
        fcvt_d_lu("fcvt.d.lu ft0,t0", FcvtDLu { frd: Ft0, rs1: T0, rm: Dyn }),
        fmv_d_x("fmv.d.x ft0,t0", FmvDX { frd: Ft0, rs1: T0 }),

        // Pseudoinstructions (50 tests)
        nop("nop", Nop {}),
        li("li t0,4", Li { rd: T0, imm: Immediate(4) }),
        mv("mv t0,t1", Mv { rd: T0, rs1: T1 }),
        not("not t0,t1", Not { rd: T0, rs1: T1 }),
        neg("neg t0,t1", Neg { rd: T0, rs1: T1 }),
        negw("negw t0,t1", Negw { rd: T0, rs1: T1 }),
        sext_w("sext.w t0,t1", SextW { rd: T0, rs1: T1 }),
        seqz("seqz t0,t1", Seqz { rd: T0, rs1: T1 }),
        snez("snez t0,t1", Snez { rd: T0, rs1: T1 }),
        sltz("sltz t0,t1", Sltz { rd: T0, rs1: T1 }),
        sgtz("sgtz t0,t1", Sgtz { rd: T0, rs1: T1 }),

        fmv_s("fmv.s ft0,ft1", FmvS { frd: Ft0, frs1: Ft1 }),
        fabs_s("fabs.s ft0,ft1", FabsS { frd: Ft0, frs1: Ft1 }),
        fneg_s("fneg.s ft0,ft1", FnegS { frd: Ft0, frs1: Ft1 }),
        fmv_d("fmv.d ft0,ft1", FmvD { frd: Ft0, frs1: Ft1 }),
        fabs_d("fabs.d ft0,ft1", FabsD { frd: Ft0, frs1: Ft1 }),
        fneg_d("fneg.d ft0,ft1", FnegD { frd: Ft0, frs1: Ft1 }),

        beqz("beqz t0,main", Beqz { rs1: T0, addr: Address(0x0) }),
        bnez("bnez t0,main", Bnez { rs1: T0, addr: Address(0x0) }),
        blez("blez t0,main", Blez { rs1: T0, addr: Address(0x0) }),
        bgez("bgez t0,main", Bgez { rs1: T0, addr: Address(0x0) }),
        bltz("bltz t0,main", Bltz { rs1: T0, addr: Address(0x0) }),
        bgtz("bgtz t0,main", Bgtz { rs1: T0, addr: Address(0x0) }),

        j("j main", J { addr: Address(0x0) }),
        jr("jr t0", Jr { rs1: T0 }),
        pseudo_jalr("jalr t0", PseudoJalr { rs1: T0 }),
        ret("ret", Ret {}),

        pseudo_fence("fence", PseudoFence {}),

        rdinstret("rdinstret t0", Rdinstret { rd: T0 }),
        rdinstreth("rdinstreth t0", Rdinstreth { rd: T0 }, ["-march=rv32gc", "-mabi=ilp32d"]),
        rdcycle("rdcycle t0", Rdcycle { rd: T0 }),
        rdcycleh("rdcycleh t0", Rdcycleh { rd: T0 }, ["-march=rv32gc", "-mabi=ilp32d"]),
        rdtime("rdtime t0", Rdtime { rd: T0 }),
        rdtimeh("rdtimeh t0", Rdtimeh { rd: T0 }, ["-march=rv32gc", "-mabi=ilp32d"]),

        csrr("csrr t0, 0x99", Csrr { rd: T0, csr: Unknown(0x99) }),
        csrw("csrw 0x99, t0", Csrw { csr: Unknown(0x99), rs1: T0 }),
        csrs("csrs 0x99, t0", Csrs { csr: Unknown(0x99), rs1: T0 }),
        csrc("csrc 0x99, t0", Csrc { csr: Unknown(0x99), rs1: T0 }),

        csrwi("csrwi 0x99, 4", Csrwi { csr: Unknown(0x99), imm: Immediate(4) }),
        csrsi("csrsi 0x99, 4", Csrsi { csr: Unknown(0x99), imm: Immediate(4) }),
        csrci("csrci 0x99, 4", Csrci { csr: Unknown(0x99), imm: Immediate(4) }),

        frcsr("frcsr t0", Frcsr { rd: T0 }),
        fscsr("fscsr t0,t1", Fscsr { rd: T0, rs1: T1 }),
        fwcsr("fscsr t0", Fwcsr { rs1: T0 }),

        frrm("frrm t0", Frrm { rd: T0 }),
        fsrm("fsrm t0,t1", Fsrm { rd: T0, rs1: T1 }),
        fwrm("fsrm t0", Fwrm { rs1: T0 }),

        frflags("frflags t0", Frflags { rd: T0 }),
        fsflags("fsflags t0,t1", Fsflags { rd: T0, rs1: T1 }),
        fwflags("fsflags t0", Fwflags { rs1: T0 }),
    }
}
