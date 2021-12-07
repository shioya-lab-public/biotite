macro_rules! _0 {
    ($addr:ident, $abi:expr, $value:ident) => {
        Value::Temp(*$addr, 0)
    };
}

macro_rules! _i {
    ($addr:ident, $abi:expr, $value:ident) => {
        match $abi {
            Abi::Ilp32 | Abi::Ilp32f | Abi::Ilp32d => Type::I32,
            Abi::Lp64 | Abi::Lp64f | Abi::Lp64d => Type::I64,
        }
    };
}

macro_rules! imm {
    ($addr:ident, $abi:expr, $value:ident) => {
        Value::Immediate(*$value)
    };
}

macro_rules! imm_12 {
    ($addr:ident, $abi:expr, $value:ident) => {
        Value::Immediate(Immediate(12))
    };
}

macro_rules! build_instructions {
    ( $addr:ident, $abi:expr, $( $inst:tt { $( $field:ident: $value:ident ),* }, )* ) => {
        {
            let temps = vec![
                Value::Temp(*$addr, 0)
            ];
            vec![
                $(
                    $inst {
                        $(
                            $field: $value!($addr, $abi, $value),
                        )*
                    },
                )*
            ]
        }
    };
}

pub(crate) use {_i, build_instructions, imm, imm_12, _0};
