#[macro_export]
macro_rules! rd {
    () => {
        RiscvRegister
    };
}

#[macro_export]
macro_rules! rs1 {
    () => {
        RiscvRegister
    };
}

#[macro_export]
macro_rules! rs2 {
    () => {
        RiscvRegister
    };
}

#[macro_export]
macro_rules! imm {
    () => {
        RiscvImmediate
    };
}

#[macro_export]
macro_rules! addr {
    () => {
        RiscvAddress
    };
}

#[macro_export]
macro_rules! define_instruction {
    ( $( $inst:ident ( $( $field:ident ),* ) ),* $(,)? ) => {
        #[derive(Debug, PartialEq, Clone)]
        pub enum RiscvInstruction {
            $(
                $inst {
                    label: Option<String>,
                    address: RiscvAddress,
                    $(
                        $field: $field!(),
                    )*
                    comment: Option<String>,
                },
            )*
        }

        impl RiscvInstruction {
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
macro_rules! define_regex {
    ( $( $name:ident ( $( $arg:tt ),* ) ),* $(,)? ) => {
        lazy_static! {
            $(
                pub static ref $name: Regex = Regex::new(&format!($($arg),*)).unwrap();
            )*
        }
    };
}

#[macro_export]
macro_rules! build_instruction {
    ( $line:ident, $label:ident, $( $regex:path => $inst:ident ( $( $field:ident ),* ) ),* $(,)? ) => {
        use crate::riscv_isa::{RiscvImmediate, RiscvAddress, RiscvRegister, FromStr};
        match $line {
            $(
                line if $regex.is_match(line) => {
                    let caps = $regex.captures(line).unwrap();
                    $inst {
                        $label,
                        address: RiscvAddress::from_str(&caps["address"]),
                        $(
                            $field: <$field!()>::from_str(
                                caps.name(stringify!($field))
                                    .map(|m| m.as_str())
                                    .unwrap_or("default")
                            ),
                        )*
                        comment: caps.name("comm").map(|m| m.as_str().to_string()),
                    }
                }
            )*
            line => panic!("Unknown RISC-V instruction `{}`", line),
        }
    };
}

#[macro_export]
macro_rules! build_test {
    ( $( $name:ident ( $source:literal, $inst:ident { $( $field:ident: $value:tt ),* $(,)? } ) ),* $(,)? ) => {
        $(
            #[test]
            fn $name() {
                let inst = super::build_instruction($source, None);
                let expected = $inst {
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
