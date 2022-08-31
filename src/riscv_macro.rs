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
        Csr
    };
}
macro_rules! ord {
    ("regex") => {
        ORD
    };
    ("type") => {
        Ord
    };
}

macro_rules! rm {
    ("regex") => {
        RM
    };
    ("type") => {
        Rm
    };
}

macro_rules! define_inst {
    ( $( $inst:ident ( $regex:literal $(, $field:ident )* ), )* ) => {
        use lazy_static::lazy_static;
        use regex::{Regex, RegexSet};

        const RD: &str = r"(?P<rd>[[:alpha:]][[:alnum:]]+)";
        const RS1: &str = r"(?P<rs1>[[:alpha:]][[:alnum:]]+)";
        const RS2: &str = r"(?P<rs2>[[:alpha:]][[:alnum:]]+)";
        const FRD: &str = r"(?P<frd>[[:alpha:]][[:alnum:]]+)";
        const FRS1: &str = r"(?P<frs1>[[:alpha:]][[:alnum:]]+)";
        const FRS2: &str = r"(?P<frs2>[[:alpha:]][[:alnum:]]+)";
        const FRS3: &str = r"(?P<frs3>[[:alpha:]][[:alnum:]]+)";
        const IMM: &str = r"(?P<imm>(-|(0x))?[[:xdigit:]]+)";
        const ADDR: &str = r"(?P<addr>[[:xdigit:]]+)";
        const CSR: &str = r"(?P<csr>[[:alpha:]]+|(0x[[:xdigit:]]+))";
        const ORD: &str = r"(\.(?P<ord>[[:alpha:]]+))?";
        const RM: &str = r"(,(?P<rm>[[:alpha:]]+))?";
        const INST_ADDR: &str = r"(?P<inst_addr>[[:xdigit:]]+)";
        const INST_BYTE: &str = r"(?P<inst_byte>[[:xdigit:]]+)";

        lazy_static! {
            static ref REGEXES: Vec<(&'static str, Regex)> = vec![
                $(
                    (
                        stringify!($inst),
                        Regex::new(&format!(
                            concat!(r"\s+{}:\s+{}\s+", $regex),
                            INST_ADDR, INST_BYTE, $( $field!("regex") ),*
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

        #[derive(Debug, PartialEq, Clone, Copy)]
        pub enum Inst {
            $(
                $inst {
                    inst_addr: Addr,
                    compressed: bool,
                    $(
                        $field: $field!("type"),
                    )*
                },
            )*
        }

        impl Inst {
            pub fn new(line: &str) -> Inst {
                use Inst::*;

                let matches: Vec<_> = REGEX_SET.matches(line).into_iter().collect();
                if matches.is_empty() {
                    panic!("Unknown instruction `{line}`");
                }
                let (name, regex) = &REGEXES[matches[0]];
                let caps = regex.captures(line).unwrap();
                match *name {
                    $(
                        stringify!($inst) => $inst {
                            inst_addr: Addr::new(&caps["inst_addr"]),
                            compressed: caps["inst_byte"].len() == 4,
                            $(
                                $field: <$field!("type")>::new(
                                    caps.name(stringify!($field))
                                        .map(|m| m.as_str())
                                        .unwrap_or_default()
                                ),
                            )*
                        },
                    )*
                    _ => unreachable!(),
                }
            }

            pub fn inst_addr(&self) -> Addr {
                use Inst::*;

                match self {
                    $(
                        $inst { inst_addr, .. } => *inst_addr,
                    )*
                }
            }

            pub fn compressed(&self) -> bool {
                use Inst::*;

                match self {
                    $(
                        $inst { compressed, .. } => *compressed,
                    )*
                }
            }
        }
    };
}

pub(crate) use {addr, csr, define_inst, frd, frs1, frs2, frs3, imm, ord, rd, rm, rs1, rs2};
