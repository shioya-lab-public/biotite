macro_rules! _0 {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        Value::Temp(*$addr, 0)
    };
}

macro_rules! _1 {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        Value::Temp(*$addr, 1)
    };
}

macro_rules! _2 {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        Value::Temp(*$addr, 2)
    };
}

macro_rules! _3 {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        Value::Temp(*$addr, 3)
    };
}

macro_rules! _4 {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        Value::Temp(*$addr, 4)
    };
}

macro_rules! _5 {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        Value::Temp(*$addr, 5)
    };
}

macro_rules! _i {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        match $abi {
            Abi::Ilp32 | Abi::Ilp32f | Abi::Ilp32d => Type::I32,
            Abi::Lp64 | Abi::Lp64f | Abi::Lp64d => Type::I64,
        }
    };
}

macro_rules! _i8 {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        Type::I8
    };
}

macro_rules! _i32 {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        Type::I32
    };
}

macro_rules! _i64 {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        Type::I64
    };
}

macro_rules! f {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        match $abi {
            Abi::Ilp32 | Abi::Lp64 => panic!("Instruction at `{}` requires a FP ABI", $addr),
            Abi::Ilp32f | Abi::Lp64f => FPType::Float,
            Abi::Ilp32d | Abi::Lp64d => FPType::Double,
        }
    };
}

macro_rules! default {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        $value
    };
}

macro_rules! targets {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        $value
    };
}

macro_rules! stk {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        Value::Address($value)
    };
}

macro_rules! ver {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        Value::Immediate(Immediate($value))
    };
}

macro_rules! address {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        Value::Address(*$value)
    };
}

macro_rules! next_pc {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        match ($addr, $raw) {
            (Address(addr), Raw(raw)) if raw.len() == 4 => Value::Address(Address(addr + 2)),
            (Address(addr), Raw(raw)) if raw.len() == 8 => Value::Address(Address(addr + 4)),
            _ => unreachable!(),
        }
    };
}

macro_rules! ra {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        Value::Register(Register::Ra)
    };
}

macro_rules! rd {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        Value::Register(*$value)
    };
}

macro_rules! rs1 {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        Value::Register(*$value)
    };
}

macro_rules! rs2 {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        Value::Register(*$value)
    };
}

macro_rules! imm {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        Value::Immediate(*$value)
    };
}

macro_rules! imm_12 {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        Value::Immediate(Immediate(12))
    };
}

macro_rules! addr {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        Value::Address(*$value)
    };
}

macro_rules! uge {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        Condition::Uge
    };
}

macro_rules! ult {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        Condition::Ult
    };
}

macro_rules! sge {
    ( $addr:expr, $raw:expr, $abi:expr, $value:ident ) => {
        Condition::Sge
    };
}

macro_rules! build_instructions {
    ( $addr:expr, $raw:expr, $abi:expr, $( $inst:tt { $( $field:ident: $value:ident ),* }, )* ) => {
        vec![
            $(
                $inst {
                    $(
                        $field: $value!($addr, $raw, $abi, $value),
                    )*
                },
            )*
        ]
    };
}

pub(crate) use {
    _i, _i32, _i64, _i8, addr, address, build_instructions, default, f, imm, imm_12, next_pc, ra,
    rd, rs1, rs2, sge, stk, targets, uge, ver, _0, _1, _2, _3, _4, _5,ult
};
