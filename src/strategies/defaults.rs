use crate::{
    default_to,
    strategies::{
        KeepResults, Widen,
        sum::{SaturatingSum, Sum},
    },
};

default_to! {
    u8 => KeepResults<SaturatingSum>,
    u16 => KeepResults<SaturatingSum>,
    u32 => KeepResults<SaturatingSum>,
    u64 => KeepResults<SaturatingSum>,
    u128 => KeepResults<SaturatingSum>,
    usize => KeepResults<SaturatingSum>,

    i8 => KeepResults<Widen<i16, Sum>>,
    i16 => KeepResults<Widen<i32, Sum>>,
    i32 => KeepResults<Widen<i64, Sum>>,
    i64 => KeepResults<Widen<i128, Sum>>,
    isize => KeepResults<Sum>,

    f32 => KeepResults<Sum>,
    f64 => KeepResults<Sum>,
}

#[cfg(feature = "ordered-float")]
default_to! {
    ordered_float::OrderedFloat<f32> => KeepResults<Sum>,
    ordered_float::OrderedFloat<f64> => KeepResults<Sum>,
}
