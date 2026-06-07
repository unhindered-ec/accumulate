use crate::strategy::{AccumulateStrategy, TotalResult};

pub struct OverflowError;

#[derive(Debug, Clone, Copy)]
pub struct CheckedSum;

pub trait CheckedAdd {
    type Accumulated: Sized + Default;

    fn checked_add_assign(
        accumulated: &mut Self::Accumulated,
        rhs: Self::Accumulated,
    ) -> Option<()>;

    fn checked_sum<I>(iter: I) -> Option<Self::Accumulated>
    where
        I: Iterator<Item = Self>;
}

impl<T> AccumulateStrategy<T> for CheckedSum
where
    T: CheckedAdd,
{
    type Error = OverflowError;
    type State = T::Accumulated;

    fn initialize() -> Self::State {
        T::Accumulated::default()
    }

    fn accumulate_into<I>(state: &mut Self::State, iter: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = T>,
    {
        let intermediary = T::checked_sum(iter).ok_or(OverflowError)?;
        T::checked_add_assign(state, intermediary).ok_or(OverflowError)?;

        Ok(())
    }
}

impl<T> TotalResult<T> for CheckedSum
where
    T: CheckedAdd,
{
    type Total = T::Accumulated;

    fn total(state: &Self::State) -> &Self::Total {
        state
    }

    fn into_total(state: Self::State) -> Self::Total {
        state
    }
}

macro_rules! impl_checked_sum_strategy {
    ($t:ty) => {
        impl CheckedAdd for $t {
            type Accumulated = $t;

            fn checked_add_assign(accumulated: &mut Self::Accumulated, rhs: Self::Accumulated) -> Option<()> {
                *accumulated = accumulated.checked_add(rhs)?;

                Some(())
            }

            fn checked_sum<I>(mut iter: I) -> Option<Self::Accumulated>
            where
                I: Iterator<Item = Self> {
                let mut accumulated = iter.next().unwrap_or_default();

                for item in iter {
                    accumulated = accumulated.checked_add(item)?;
                }

                Some(accumulated)
            }
        }
    };
    ($($t: ty),+$(,)?) => {
        $(impl_checked_sum_strategy!($t);)+
    };
}

// It's not clear what we would want the saturating sum of signed values to be,
// so we'll comment this out for now.
// impl_accumulate_strategy!(signed i8, i16, i32, i64, i128, isize);

impl_checked_sum_strategy!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);
