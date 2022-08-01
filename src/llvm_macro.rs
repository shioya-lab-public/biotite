#![allow(unused_macros)]
#![allow(unused_imports)]

macro_rules! _0 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Temp(*$addr, 0)
    };
}

macro_rules! _1 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Temp(*$addr, 1)
    };
}

macro_rules! _2 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Temp(*$addr, 2)
    };
}

macro_rules! _3 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Temp(*$addr, 3)
    };
}

macro_rules! _4 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Temp(*$addr, 4)
    };
}

macro_rules! _5 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Temp(*$addr, 5)
    };
}

macro_rules! _6 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Temp(*$addr, 6)
    };
}

macro_rules! _7 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Temp(*$addr, 7)
    };
}

macro_rules! _i1 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Type::I1
    };
}

macro_rules! _i8 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Type::I8
    };
}

macro_rules! _i16 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Type::I16
    };
}

macro_rules! _i32 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Type::I32
    };
}

macro_rules! _i64 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Type::I64
    };
}

macro_rules! _i128 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Type::I128
    };
}

macro_rules! _f {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Type::Float
    };
}

macro_rules! _d {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Type::Double
    };
}

macro_rules! default {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        $value
    };
}

macro_rules! targets {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        $value
    };
}

macro_rules! stk {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Address($value)
    };
}

macro_rules! address {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Address(*$value)
    };
}

macro_rules! next_pc {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        match ($addr, $raw) {
            (Address(addr), Raw(raw)) if raw.len() == 5 => Value::Address(Address(addr + 2)),
            (Address(addr), Raw(raw)) if raw.len() == 9 => Value::Address(Address(addr + 4)),
            _ => unreachable!(),
        }
    };
}

macro_rules! a7 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Register(Register::A7)
    };
}

macro_rules! a0 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Register(Register::A0)
    };
}

macro_rules! a1 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Register(Register::A1)
    };
}

macro_rules! a2 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Register(Register::A2)
    };
}

macro_rules! a3 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Register(Register::A3)
    };
}

macro_rules! a4 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Register(Register::A4)
    };
}

macro_rules! a5 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Register(Register::A5)
    };
}

macro_rules! rd {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Register(*$value)
    };
}

macro_rules! rs1 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Register(*$value)
    };
}

macro_rules! rs2 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Register(*$value)
    };
}

macro_rules! frd {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::FPRegister(*$value)
    };
}

macro_rules! frs1 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::FPRegister(*$value)
    };
}

macro_rules! frs2 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::FPRegister(*$value)
    };
}

macro_rules! frs3 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::FPRegister(*$value)
    };
}

macro_rules! imm {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Immediate(*$value)
    };
}

macro_rules! imm_12 {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Immediate(Immediate(12))
    };
}

macro_rules! addr {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Value::Address(*$value)
    };
}

macro_rules! eq {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Condition::Eq
    };
}

macro_rules! ne {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Condition::Ne
    };
}

macro_rules! uge {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Condition::Uge
    };
}

macro_rules! ult {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Condition::Ult
    };
}

macro_rules! sgt {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Condition::Sgt
    };
}

macro_rules! sge {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Condition::Sge
    };
}

macro_rules! slt {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Condition::Slt
    };
}

macro_rules! sle {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Condition::Sle
    };
}

macro_rules! oeq {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        FPCondition::Oeq
    };
}

macro_rules! olt {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        FPCondition::Olt
    };
}

macro_rules! ole {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        FPCondition::Ole
    };
}

macro_rules! monotonic {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Ordering::Monotonic
    };
}

macro_rules! acquire {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Ordering::Acquire
    };
}

macro_rules! release {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Ordering::Release
    };
}

macro_rules! acq_rel {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Ordering::AcqRel
    };
}

macro_rules! seq_cst {
    ( $addr:expr, $raw:expr, $value:ident ) => {
        Ordering::SeqCst
    };
}

macro_rules! build_instructions {
    ( $addr:expr, $raw:expr, $( $inst:tt { $( $field:ident: $value:ident ),* }, )* ) => {
        vec![
            $(
                $inst {
                    $(
                        $field: $value!($addr, $raw, $value),
                    )*
                },
            )*
        ]
    };
}

pub(crate) use {
    _d, _f, _i1, _i128, _i16, _i32, _i64, _i8, a0, a1, a2, a3, a4, a5, a7, acq_rel, acquire, addr,
    address, build_instructions, default, eq, frd, frs1, frs2, frs3, imm, imm_12, monotonic, ne,
    next_pc, oeq, ole, olt, rd, release, rs1, rs2, seq_cst, sge, sgt, sle, slt, stk, targets, uge,
    ult, _0, _1, _2, _3, _4, _5, _6, _7,
};
