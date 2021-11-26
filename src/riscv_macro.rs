#[macro_export]
macro_rules! addr {
    ("uppercase") => {
        ADDR
    };
    ("type") => {
        Address
    };
}

#[macro_export]
macro_rules! ord {
    ("uppercase") => {
        ORD
    };
    ("type") => {
        Ordering
    };
}

#[macro_export]
macro_rules! rd {
    ("uppercase") => {
        RD
    };
    ("type") => {
        Register
    };
}

#[macro_export]
macro_rules! rs1 {
    ("uppercase") => {
        RS1
    };
    ("type") => {
        Register
    };
}

#[macro_export]
macro_rules! rs2 {
    ("uppercase") => {
        RS2
    };
    ("type") => {
        Register
    };
}

#[macro_export]
macro_rules! rs3 {
    ("uppercase") => {
        RS3
    };
    ("type") => {
        Register
    };
}

#[macro_export]
macro_rules! imm {
    ("uppercase") => {
        IMM
    };
    ("type") => {
        Immediate
    };
}

#[macro_export]
macro_rules! define_instruction {
    ( $( $inst:ident ( $regex:literal $(, $field:ident )* ), )* ) => {
        const ADDR: &str = r"(?P<addr>[[:xdigit:]]+)";
        const ORD: &str = r"(?P<ord>[[:alpha:]]+)";
        const RD: &str = r"(?P<rd>[[:alnum:]]+)";
        const RS1: &str = r"(?P<rs1>[[:alnum:]]+)";
        const RS2: &str = r"(?P<rs2>[[:alnum:]]+)";
        const RS3: &str = r"(?P<rs3>[[:alnum:]]+)";
        const IMM: &str = r"(?P<imm>-?[[:xdigit:]]+)";
        const CMT: &str = r"(?P<cmt>.+)";

        lazy_static! {
            static ref REGEXES: Vec<(&'static str, Regex)> = vec![
                $(
                    (
                        stringify!($inst),
                        Regex::new(&format!(
                            concat!(r"{}:\s+\S+\s+", $regex, r"(\s+{})?"),
                            ADDR, $( $field!("uppercase"), )* CMT
                        )).unwrap()
                    ),
                )*
            ];
        }

        lazy_static! {
            static ref REGEX_SET: RegexSet = RegexSet::new(vec![
                $(
                    format!(
                        concat!(r"{}:\s+\S+\s+", $regex, r"(\s+{})?"),
                        ADDR, $( $field!("uppercase"), )* CMT
                    ),
                )*
            ]).unwrap();
        }

        #[derive(Debug, PartialEq)]
        pub enum Instruction {
            $(
                $inst {
                    label: Option<String>,
                    address: Address,
                    $(
                        $field: $field!("type"),
                    )*
                    comment: Option<String>,
                },
            )*
        }

        impl Instruction {
            pub fn new(line: &str, label: Option<String>) -> Instruction {
                use Instruction::*;

                let matches: Vec<_> = REGEX_SET.matches(line).into_iter().collect();
                if matches.is_empty() {
                    panic!("Unknown instruction: {}", line);
                }
                let (inst, regex) = &REGEXES[matches[0]];
                let caps = regex.captures(line).unwrap();

                match *inst {
                    $(
                        stringify!($inst) => {
                            $inst {
                                label,
                                address: Address::new(&caps["addr"]),
                                $(
                                    $field: <$field!("type")>::new(&caps[stringify!($field)]),
                                )*
                                comment: Some(caps["cmt"].to_string()),
                            }
                        }
                    )*
                    _ => unreachable!(),
                }
            }

            pub fn label(&self) -> &Option<String> {
                use Instruction::*;

                match self {
                    $(
                        $inst { label, .. } => label,
                    )*
                }
            }

            pub fn address(&self) -> &Address {
                use Instruction::*;

                match self {
                    $(
                        $inst { address, .. } => address,
                    )*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! build_test {
    ( $( $func:ident ( $source:literal, $inst:ident { $( $field:ident: $value:expr ),* } ), )* ) => {
        $(
            #[test]
            fn $func() {
                let source = concat!("
                    main:
                        ", $source, "
                        ret
                ");
                let source = compile_and_dump(source);
                let program = Parser::new(&source).run();
                assert_eq!(
                    program,
                    Program {
                        functions: vec![Function {
                            name: String::from("main"),
                            basic_blocks: vec![BasicBlock {
                                instructions: vec![
                                    Instruction::$inst {
                                        label: Some(String::from("main")),
                                        address: Address(0x0),
                                        $(
                                            $field: $value,
                                        )*
                                        comment: None,
                                    },
                                    Instruction::Ret {
                                        label: None,
                                        address: Address(0x4),
                                        comment: None,
                                    }
                                ],
                                continue_target: None,
                                jump_target: None,
                            }],
                            indirect_targets: HashMap::new(),
                        }],
                        data: HashMap::new(),
                    }
                );
            }
        )*
    };
}
