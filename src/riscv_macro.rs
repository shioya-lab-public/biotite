macro_rules! define_insts {
    ( $( $inst:ident ( $regex:literal $(, $field:ident )* ), )* ) => {
        use once_cell::sync::Lazy;
        use regex::{Regex, RegexSet};

        const RD: &str = r"(?P<rd>[[:alpha:]][[:alnum:]]+)";
        const RS1: &str = r"(?P<rs1>[[:alpha:]][[:alnum:]]+)";
        const RS2: &str = r"(?P<rs2>[[:alpha:]][[:alnum:]]+)";
        const FRD: &str = r"(?P<frd>[[:alpha:]][[:alnum:]]+)";
        const FRS1: &str = r"(?P<frs1>[[:alpha:]][[:alnum:]]+)";
        const FRS2: &str = r"(?P<frs2>[[:alpha:]][[:alnum:]]+)";
        const FRS3: &str = r"(?P<frs3>[[:alpha:]][[:alnum:]]+)";
        const IMM: &str = r"(?P<imm>-?[[:digit:]]+)";
        const ADDR: &str = r"0x(?P<addr>[[:xdigit:]]+)";
        const CSR: &str = r"(?P<csr>[[:alpha:]]+)";
        const MO: &str = r"(\.(?P<mo>[[:alpha:]]+))?";
        const RM: &str = r"(,\s+(?P<rm>[[:alpha:]]+))?";
        const INST_ADDR: &str = r"(?P<inst_addr>[[:xdigit:]]+)";
        const INST_BYTE: &str = r"(?P<inst_byte>([[:xdigit:]]{2} )+)";
        const SYM: &str = r"(?P<sym>.*)";

        static REGEXES: Lazy<Vec<(&str, Regex)>> = Lazy::new(|| {
            vec![
                $(
                    (
                        stringify!($inst),
                        Regex::new(&format!(
                            concat!(r"{}:\s+{}\s+", $regex, r"\s*{}"),
                            INST_ADDR, INST_BYTE, $( $field!("regex"), )* SYM
                        )).unwrap()
                    ),
                )*
            ]
        });

        static REGEX_SET: Lazy<RegexSet> = Lazy::new(|| {
            RegexSet::new(REGEXES.iter().map(|(_, re)| re.as_str())).unwrap()
        });

        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum Inst {
            $(
                $inst {
                    address: Addr,
                    is_compressed: bool,
                    $( $field: $field!("type"), )*
                    symbol: Option<String>,
                },
            )*
        }

        impl Inst {
            pub fn new(line: &str) -> Self {
                use Inst::*;

                let matches: Vec<_> = REGEX_SET.matches(line).into_iter().collect();
                if matches.is_empty() {
                    panic!("Unknown RISC-V instruction `{line}`");
                }
                let (name, re) = &REGEXES[matches[0]];
                let caps = re.captures(line).unwrap();
                match *name {
                    $(
                        stringify!($inst) => $inst {
                            address: Addr::new(&caps["inst_addr"]),
                            is_compressed: caps["inst_byte"].len() == 6,
                            $(
                                $field: <$field!("type")>::new(
                                    caps.name(stringify!($field))
                                        .map(|m| m.as_str())
                                        .unwrap_or_default()
                                ),
                            )*
                            symbol: caps.name("sym").map(|m| m.as_str().to_string()),
                        },
                    )*
                    _ => unreachable!(),
                }
            }

            pub fn address(&self) -> Addr {
                use Inst::*;

                match self {
                    $( $inst { address, .. } => *address, )*
                }
            }

            pub fn is_compressed(&self) -> bool {
                use Inst::*;

                match self {
                    $( $inst { is_compressed, .. } => *is_compressed, )*
                }
            }

            pub fn symbol(&self) -> Option<&str> {
                use Inst::*;

                match self {
                    $( $inst { symbol, .. } => symbol.as_deref(), )*
                }
            }
        }
    };
}

macro_rules! rd {
    ("regex") => {
        RD
    };
    ("type") => {
        Reg
    };
}

macro_rules! rs1 {
    ("regex") => {
        RS1
    };
    ("type") => {
        Reg
    };
}

macro_rules! rs2 {
    ("regex") => {
        RS2
    };
    ("type") => {
        Reg
    };
}

macro_rules! frd {
    ("regex") => {
        FRD
    };
    ("type") => {
        FReg
    };
}

macro_rules! frs1 {
    ("regex") => {
        FRS1
    };
    ("type") => {
        FReg
    };
}

macro_rules! frs2 {
    ("regex") => {
        FRS2
    };
    ("type") => {
        FReg
    };
}

macro_rules! frs3 {
    ("regex") => {
        FRS3
    };
    ("type") => {
        FReg
    };
}

macro_rules! imm {
    ("regex") => {
        IMM
    };
    ("type") => {
        Imm
    };
}

macro_rules! addr {
    ("regex") => {
        ADDR
    };
    ("type") => {
        Addr
    };
}

macro_rules! csr {
    ("regex") => {
        CSR
    };
    ("type") => {
        CSR
    };
}

macro_rules! mo {
    ("regex") => {
        MO
    };
    ("type") => {
        MO
    };
}

macro_rules! rm {
    ("regex") => {
        RM
    };
    ("type") => {
        RM
    };
}

pub(crate) use {addr, csr, define_insts, frd, frs1, frs2, frs3, imm, mo, rd, rm, rs1, rs2};
