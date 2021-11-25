#[macro_export]
macro_rules! ord {
    ("uppercase") => {
        ORD
    };
    ("type") => {
        RiscvOrdering
    };
}

#[macro_export]
macro_rules! rd {
    ("uppercase") => {
        RD
    };
    ("type") => {
        RiscvRegister
    };
}

#[macro_export]
macro_rules! rs1 {
    ("uppercase") => {
        RS1
    };
    ("type") => {
        RiscvRegister
    };
}

#[macro_export]
macro_rules! rs2 {
    ("uppercase") => {
        RS2
    };
    ("type") => {
        RiscvRegister
    };
}

#[macro_export]
macro_rules! rs3 {
    ("uppercase") => {
        RS3
    };
    ("type") => {
        RiscvRegister
    };
}

#[macro_export]
macro_rules! imm {
    ("uppercase") => {
        IMM
    };
    ("type") => {
        RiscvImmediate
    };
}

#[macro_export]
macro_rules! addr {
    ("uppercase") => {
        ADDR
    };
    ("type") => {
        RiscvAddress
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

        #[derive(Debug, PartialEq, Clone)]
        pub enum RiscvInstruction {
            $(
                $inst {
                    label: Option<String>,
                    address: RiscvAddress,
                    $(
                        $field: $field!("type"),
                    )*
                    comment: Option<String>,
                },
            )*
        }

        impl RiscvInstruction {
            pub fn new(line: &str, label: Option<String>) -> RiscvInstruction {
                use RiscvInstruction::*;

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
                                address: RiscvAddress::new(&caps["addr"]),
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
                use RiscvInstruction::*;

                match self {
                    $(
                        $inst { label, .. } => label,
                    )*
                }
            }

            pub fn address(&self) -> &RiscvAddress {
                use RiscvInstruction::*;

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
                let inst = RiscvInstruction::new(concat!("a1:	a1                	", $source), None);
                let expected = $inst {
                    address: 0xa1.into(),
                    label: None,
                    $(
                        $field: $value,
                    )*
                    comment: None,
                };
                assert_eq!(inst, expected);
            }
        )*
    };
}
