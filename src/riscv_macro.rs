macro_rules! define_insts {
    ( $( $inst:ident ( $regex:literal $(, $field:ident )* ), )* ) => {
        use std::sync::LazyLock;
        use regex::{Regex, RegexSet};

        const RD: &str = r"(?<rd>[[:alpha:]][[:alnum:]]+)";
        const RS1: &str = r"(?<rs1>[[:alpha:]][[:alnum:]]+)";
        const RS2: &str = r"(?<rs2>[[:alpha:]][[:alnum:]]+)";
        const FRD: &str = r"(?<frd>[[:alpha:]][[:alnum:]]+)";
        const FRS1: &str = r"(?<frs1>[[:alpha:]][[:alnum:]]+)";
        const FRS2: &str = r"(?<frs2>[[:alpha:]][[:alnum:]]+)";
        const FRS3: &str = r"(?<frs3>[[:alpha:]][[:alnum:]]+)";
        const IMM: &str = r"(?<imm>-?[[:digit:]]+)";
        const ADDR: &str = r"0x(?<addr>[[:xdigit:]]+)";
        const CSR: &str = r"(?<csr>[[:alpha:]]+)";
        const MO: &str = r"(\.(?<mo>[[:alpha:]]+))?";
        const RM: &str = r"(,\s+(?<rm>[[:alpha:]]+))?";
        const INST_ADDR: &str = r"(?<inst_addr>[[:xdigit:]]+)";
        const INST_BYTE: &str = r"(?<inst_byte>([[:xdigit:]]{2} )+)";
        const SYM: &str = r"(?<sym>.*)";
        static REGEXES: LazyLock<Vec<(&str, Regex)>> = LazyLock::new(|| {
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
        static REGEX_SET: LazyLock<RegexSet> = LazyLock::new(|| {
            RegexSet::new(REGEXES.iter().map(|(_, re)| re.as_str())).unwrap()
        });

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
        pub enum Inst {
            $(
                $inst {
                    address: Addr,
                    is_compressed: bool,
                    $(
                        $field: $field!("type"),
                    )*
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
                    $(
                        $inst { address, .. } => *address,
                    )*
                }
            }

            pub fn is_compressed(&self) -> bool {
                use Inst::*;

                match self {
                    $(
                        $inst { is_compressed, .. } => *is_compressed,
                    )*
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
        Csr
    };
}

macro_rules! mo {
    ("regex") => {
        MO
    };
    ("type") => {
        Mo
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

macro_rules! regex {
    ( $re:literal ) => {{
        use regex::Regex;
        use std::sync::OnceLock;

        static RE: OnceLock<Regex> = OnceLock::new();
        RE.get_or_init(|| Regex::new($re).unwrap())
    }};
}

pub(crate) use {addr, csr, define_insts, frd, frs1, frs2, frs3, imm, mo, rd, regex, rm, rs1, rs2};
