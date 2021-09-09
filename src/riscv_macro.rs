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
    ( $( $inst:ident ( $repr:literal, $regex:literal $(, $field:ident )* ), )* ) => {
        const ADDRESS: &str = r"(?P<address>[[:xdigit:]]+)";
        const RD: &str = r"(?P<rd>\S+)";
        const RS1: &str = r"(?P<rs1>\S+)";
        const RS2: &str = r"(?P<rs2>\S+)";
        const IMM: &str = r"(?P<imm>\S+)";
        const ADDR: &str = r"(?P<addr>[[:xdigit:]]+)";
        const COMMENT: &str = r"(?P<comment>\s+.+)?";
        lazy_static! {
            static ref REGEX: Vec<(&'static str, Regex)> = vec![
                $(
                    (
                        stringify!($inst),
                        Regex::new(&format!(
                            concat!(r"{}:.+?", $repr, r"\s*",$regex, r"{}"),
                            ADDRESS, $( $field!("uppercase"), )* COMMENT
                        ))
                        .unwrap()
                    ),
                )*
            ];
        }

        #[derive(Debug, PartialEq)]
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
                lazy_static! {
                    static ref SET: RegexSet =
                        RegexSet::new(REGEX.clone().into_iter().map(|(_, r)| r.to_string())).unwrap();
                }

                let matches: Vec<_> = SET.matches(line).into_iter().collect();
                if matches.is_empty() {
                    panic!("Unknown instruction: {}", line);
                }
                let (inst, regex) = &REGEX[matches[0]];
                let caps = regex.captures(line).unwrap();
                match *inst {
                    $(
                        stringify!($inst) => {
                            $inst {
                                label,
                                address: RiscvAddress::from_str_radix(caps.name("address").unwrap().as_str(), 16).unwrap(),
                                $(
                                    $field: <$field!("type")>::new(caps.name(stringify!($field)).unwrap().as_str()),
                                )*
                                comment: caps.name("comment").map(|m| m.as_str().to_string()),
                            }
                        }
                    )*
                    _ => unreachable!(),
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
                    address: 0xa1,
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
