macro_rules! _0 {
    ( $addr:expr, $abi:expr, $value:ident ) => {
        Value::Temp(*$addr, 0)
    };
}

macro_rules! _1 {
    ( $addr:expr, $abi:expr, $value:ident ) => {
        Value::Temp(*$addr, 1)
    };
}

macro_rules! _i {
    ( $addr:expr, $abi:expr, $value:ident ) => {
        match $abi {
            Abi::Ilp32 | Abi::Ilp32f | Abi::Ilp32d => Type::I32,
            Abi::Lp64 | Abi::Lp64f | Abi::Lp64d => Type::I64,
        }
    };
}

macro_rules! _f {
    ( $addr:expr, $abi:expr, $value:ident ) => {
        match $abi {
            Abi::Ilp32 | Abi::Lp64 => panic!("Instruction at `{}` requires a FP ABI", $addr),
            Abi::Ilp32f | Abi::Lp64f => FPType::Float,
            Abi::Ilp32d | Abi::Lp64d => FPType::Double,
        }
    };
}

macro_rules! address {
    ( $addr:expr, $abi:expr, $value:ident ) => {
        Value::Address(*$value)
    };
}

macro_rules! rd {
    ( $addr:expr, $abi:expr, $value:ident ) => {
        Value::Register(*$value)
    };
}

macro_rules! imm {
    ( $addr:expr, $abi:expr, $value:ident ) => {
        Value::Immediate(*$value)
    };
}

macro_rules! imm_12 {
    ( $addr:expr, $abi:expr, $value:ident ) => {
        Value::Immediate(Immediate(12))
    };
}

macro_rules! build_instructions {
    ( $addr:expr, $abi:expr, $( $inst:tt { $( $field:ident: $value:ident ),* }, )* ) => {
        vec![
            $(
                $inst {
                    $(
                        $field: $value!($addr, $abi, $value),
                    )*
                },
            )*
        ]
    };
}

pub(crate) use {_f, _i, address, build_instructions, imm, imm_12, rd, _0, _1};
