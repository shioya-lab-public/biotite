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
macro_rules! frd {
    ("uppercase") => {
        FRD
    };
    ("type") => {
        FPRegister
    };
}

#[macro_export]
macro_rules! frs1 {
    ("uppercase") => {
        FRS1
    };
    ("type") => {
        FPRegister
    };
}

#[macro_export]
macro_rules! frs2 {
    ("uppercase") => {
        FRS2
    };
    ("type") => {
        FPRegister
    };
}

#[macro_export]
macro_rules! frs3 {
    ("uppercase") => {
        FRS3
    };
    ("type") => {
        FPRegister
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
macro_rules! addr {
    ("uppercase") => {
        ADDR
    };
    ("type") => {
        Address
    };
}

#[macro_export]
macro_rules! csr {
    ("uppercase") => {
        CSR
    };
    ("type") => {
        Csr
    };
}

#[macro_export]
macro_rules! rm {
    ("uppercase") => {
        RM
    };
    ("type") => {
        Rounding
    };
}

#[macro_export]
macro_rules! iorw {
    ("uppercase") => {
        IORW
    };
    ("type") => {
        Iorw
    };
}

#[macro_export]
macro_rules! define_instruction {
    ( $( $inst:ident ( $regex:literal $(, $field:ident )* ), )* ) => {
        const ADDRESS: &str = r"(?P<address>[[:xdigit:]]+)";
        const ORD: &str = r"(\.(?P<ord>[[:alpha:]]+))?";
        const RD: &str = r"(?P<rd>[[:alpha:]][[:alnum:]]+)";
        const RS1: &str = r"(?P<rs1>[[:alpha:]][[:alnum:]]+)";
        const RS2: &str = r"(?P<rs2>[[:alpha:]][[:alnum:]]+)";
        const FRD: &str = r"(?P<frd>[[:alpha:]][[:alnum:]]+)";
        const FRS1: &str = r"(?P<frs1>[[:alpha:]][[:alnum:]]+)";
        const FRS2: &str = r"(?P<frs2>[[:alpha:]][[:alnum:]]+)";
        const FRS3: &str = r"(?P<frs3>[[:alpha:]][[:alnum:]]+)";
        const IMM: &str = r"(?P<imm>(-|(0x))?[[:xdigit:]]+)";
        const ADDR: &str = r"(?P<addr>[[:xdigit:]]+)";
        const CSR: &str = r"(?P<csr>[[:alpha:]]+)";
        const RM: &str = r"(\.(?P<rm>[[:alpha:]]+))?";
        const IORW: &str = r"((\.|\s+)(?P<iorw>((tso)|([iorw]+,[iorw]+))))?";
        const CMT: &str = r"(\s+(?P<cmt>.+))?";

        lazy_static! {
            static ref REGEXES: Vec<(&'static str, Regex)> = vec![
                $(
                    (
                        stringify!($inst),
                        Regex::new(&format!(
                            concat!(r"{}:\s+\S+\s+", $regex, r"{}"),
                            ADDRESS, $( $field!("uppercase"), )* CMT
                        )).unwrap()
                    ),
                )*
            ];
        }

        lazy_static! {
            static ref REGEX_SET: RegexSet = RegexSet::new(
                REGEXES.iter().map(|(_, r)| r.as_str())
            ).unwrap();
        }

        #[derive(Debug, PartialEq)]
        pub enum Instruction {
            $(
                $inst {
                    address: Address,
                    $(
                        $field: $field!("type"),
                    )*
                    comment: Option<String>,
                },
            )*
        }

        impl Instruction {
            pub fn new(line: &str) -> Instruction {
                use Instruction::*;

                let matches: Vec<_> = REGEX_SET.matches(line).into_iter().collect();
                if matches.is_empty() {
                    panic!("Unknown instruction: {}", line);
                }
                let (inst, regex) = &REGEXES[matches[0]];
                let caps = regex.captures(line).unwrap();

                match *inst {
                    $(
                        stringify!($inst) => $inst {
                            address: Address::new(&caps["address"]),
                            $(
                                $field: <$field!("type")>::new(
                                    caps.name(stringify!($field))
                                        .map(|m| m.as_str())
                                        .unwrap_or_default(),
                                ),
                            )*
                            comment: caps.name("cmt").map(|m| m.as_str().to_string()),
                        },
                    )*
                    _ => unreachable!(),
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
    ( $( $func:ident ( $source:literal,
        $inst:ident { $( $field:ident: $value:expr ),* }
        $(, [$march:literal, $mabi:literal] )?
    ), )* ) => {
        $(
            #[test]
            fn $func() {
                let disasm = compile_and_dump(concat!($source, "\n"), &vec![$($march, $mabi)?]);
                let inst = Instruction::new(disasm.lines().last().unwrap());
                assert_eq!(
                    inst,
                    $inst {
                        address: Address(0),
                        $(
                            $field: $value,
                        )*
                        comment: None,
                    }
                );
            }
        )*
    };
}
