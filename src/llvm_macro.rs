//! LLVM-related macros.

macro_rules! trans_inst {
    ( $scrutinee: ident,
        $(
            $rv_inst:tt { $( $rv_field:ident ),* } => {
                $(
                    $inst:tt { $( $field:ident: $value:tt ),* },
                )*
            }
        )*
    ) => {
        match $scrutinee {
            $(
                rv::Inst::$rv_inst { address, is_compressed, $( $rv_field, )* .. } => {
                    vec![
                        $(
                            Inst::$inst {
                                $(
                                    $field: expand_value!($value, address, is_compressed),
                                )*
                            },
                        )*
                    ]
                }
            )*
        }
    };
}

macro_rules! expand_value {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        $value!($value, $address, $is_compressed)
    };
    ( $value:block, $address:expr, $is_compressed:expr ) => {
        $value
    };
}

macro_rules! i_1 {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Type::I1
    };
}

macro_rules! i_8 {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Type::I8
    };
}

macro_rules! i_16 {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Type::I16
    };
}

macro_rules! i_32 {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Type::I32
    };
}

macro_rules! i_64 {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Type::I64
    };
}

macro_rules! i_128 {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Type::I128
    };
}

macro_rules! f {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Type::Float
    };
}

macro_rules! d {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Type::Double
    };
}

macro_rules! rd {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Value::Reg($value)
    };
}

macro_rules! rs1 {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Value::Reg($value)
    };
}

macro_rules! rs2 {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Value::Reg($value)
    };
}

macro_rules! frd {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Value::FReg($value)
    };
}

macro_rules! frs1 {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Value::FReg($value)
    };
}

macro_rules! frs2 {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Value::FReg($value)
    };
}

macro_rules! frs3 {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Value::FReg($value)
    };
}

macro_rules! rs {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Value::RS
    };
}

macro_rules! imm {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Value::Imm($value)
    };
}

macro_rules! addr {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Value::Addr($value)
    };
}

macro_rules! _0 {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Value::Temp($address, 0)
    };
}
macro_rules! _1 {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Value::Temp($address, 1)
    };
}

macro_rules! _2 {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Value::Temp($address, 2)
    };
}

macro_rules! _3 {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Value::Temp($address, 3)
    };
}

macro_rules! _4 {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Value::Temp($address, 4)
    };
}

macro_rules! _5 {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Value::Temp($address, 5)
    };
}

macro_rules! _6 {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Value::Temp($address, 6)
    };
}

macro_rules! _7 {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Value::Temp($address, 7)
    };
}

macro_rules! _8 {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Value::Temp($address, 8)
    };
}

macro_rules! _9 {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Value::Temp($address, 9)
    };
}

macro_rules! mo {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        match $value {
            rv::Mo::Mono => Mo::Monotonic,
            rv::Mo::Aq => Mo::Acquire,
            rv::Mo::Rl => Mo::Release,
            rv::Mo::AqRl => Mo::SeqCst,
        }
    };
}

macro_rules! rm {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        match $value {
            rv::Rm::Rne => Rm::Tonearest,
            rv::Rm::Rtz => Rm::Towardzero,
            rv::Rm::Rdn => Rm::Downward,
            rv::Rm::Rup => Rm::Upward,
            rv::Rm::Rmm => Rm::Tonearestaway,
            rv::Rm::Dyn => Rm::Dynamic,
        }
    };
}

macro_rules! pc {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {
        Value::Addr($address)
    };
}

macro_rules! next_pc {
    ( $value:ident, $address:expr, $is_compressed:expr ) => {{
        let rv::Addr(addr) = $address;
        let len = if $is_compressed { 2 } else { 4 };
        Value::Addr(rv::Addr(addr + len))
    }};
}

pub(crate) use {
    _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, addr, d, expand_value, f, frd, frs1, frs2, frs3, i_1,
    i_8, i_16, i_32, i_64, i_128, imm, mo, next_pc, pc, rd, rm, rs, rs1, rs2, trans_inst,
};
