use std::convert::Infallible;

use crate::strategy::{AccumulateStrategy, TotalResult};

#[derive(Debug, Clone, Copy)]
pub struct SaturatingSum;

pub trait SaturatingAdd {
    type Accumulated: Sized + Default;

    fn saturating_add_assign(accumulated: &mut Self::Accumulated, rhs: Self::Accumulated);

    fn saturating_sum<I>(iter: I) -> Self::Accumulated
    where
        I: Iterator<Item = Self>;
}

impl<T> AccumulateStrategy<T> for SaturatingSum
where
    T: SaturatingAdd,
{
    type Error = Infallible;
    type State = T::Accumulated;

    fn initialize() -> Self::State {
        Self::State::default()
    }

    fn accumulate_into<I>(state: &mut Self::State, iter: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = T>,
    {
        T::saturating_add_assign(state, T::saturating_sum(iter));

        Ok(())
    }
}

impl<T> TotalResult<T> for SaturatingSum
where
    T: SaturatingAdd,
{
    type Total = T::Accumulated;

    fn total(state: &Self::State) -> &Self::Total {
        state
    }

    fn into_total(state: Self::State) -> Self::Total {
        state
    }
}

macro_rules! impl_saturating_sum_strategy {
    (unsigned $t:ty) => {
        impl SaturatingAdd for $t {
            type Accumulated = $t;

            fn saturating_add_assign(accumulated: &mut Self::Accumulated, rhs: Self::Accumulated) {
                *accumulated = accumulated.saturating_add(rhs);
            }

            fn saturating_sum<I>(iter: I) -> Self::Accumulated
            where
                I: Iterator<Item = Self> {
                let std::num::Saturating(x) = iter.map(|x| std::num::Saturating(x)).sum();

                x
            }
        }
    };
    ($s: tt $($t: ty),+$(,)?) => {
        $(impl_saturating_sum_strategy!($s $t);)+
    };
}

// It's not clear what we would want the saturating sum of signed values to be,
// so we'll comment this out for now.
// impl_accumulate_strategy!(signed i8, i16, i32, i64, i128, isize);

impl_saturating_sum_strategy!(unsigned u8, u16, u32, u64, u128, usize);
