use std::convert::Infallible;

use super::{strategy::AccumulateStrategy, total::TotalResult};
// use crate::performance::{error_value::ErrorValue, score_value::Wrapper};

/// Accumulation strategy that uses saturating summation for its total.
///
/// This is currently only implemented for unsigned types since the behavior for
/// signed types is not well defined. It's unclear what the desired behavior
/// should be when, for example, adding two large signed types saturates, and
/// then we add a negative value to that saturated value.
///
/// # Implementing
///
/// To implement this strategy for a new type `T`, you need to implement most or
/// all of these traits for `T`:
///
/// - `AccumulateStrategy<T> for SaturatingSum`
/// - `TotalResult<T> for SaturatingSum`
/// - `IndividualResults<T> for SaturatingSum`
/// - `IndexResults<T> for SaturatingSum`
///
/// In most cases it is probably fine to skip implementing `IndividualResults`
/// and `IndexResults` and rely on `KeepResults<SaturatingSum>` if you need
/// access to individual results.
///
/// If in your specific implementation of `SaturatingSum` you need to keep all
/// results to calculate a total anyways, it might make sense to also implement
/// the `IndividualResults` and `IndexResults` traits to avoid needing a
/// `KeepResults` and storing them twice.
///
/// In particular, if you have a general wrapper type `Wrapper<T>` that forwards
/// its accumulate implementation to an inner type `T` that implements these
/// traits, you will want to implement these for your wrapper type as well.
/// See the example below.
///
/// ```
/// # use unhindered_accumulate::{results::{IndividualResults, IndexResults}, strategy::AccumulateStrategy, saturating_sum::SaturatingSum, total::TotalResult};
/// // #[derive(ref_cast::RefCast)]
/// #[repr(transparent)]
/// struct Wrapper<T: ?Sized>(pub T);
///
/// impl<T> Wrapper<T> {
///     fn new(inner: T) -> Self {
///         Self(inner)
///     }
///
///     // Deriving `ref_cast::RefCast` will provide this method
///     // for you, but does add a dependency on that crate.
///     fn ref_cast(_: &T) -> &Self {
///         todo!();
///     }
/// }
///
/// impl<T> AccumulateStrategy<Wrapper<T>> for SaturatingSum
/// where
///     Self: AccumulateStrategy<T>,
/// {
///     type Error = <Self as AccumulateStrategy<T>>::Error;
///     type State = <Self as AccumulateStrategy<T>>::State;
///
///     fn initialize() -> Self::State {
///         <Self as AccumulateStrategy<T>>::initialize()
///     }
///
///     fn accumulate_into<I>(state: &mut Self::State, iter: I) -> Result<(), Self::Error>
///     where
///         I: Iterator<Item = Wrapper<T>>,
///     {
///         <Self as AccumulateStrategy<T>>::accumulate_into(state, iter.map(|sv| sv.0))
///     }
///
///     fn accumulate<I>(iter: I) -> Result<Self::State, Self::Error>
///     where
///         I: Iterator<Item = Wrapper<T>>,
///     {
///         <Self as AccumulateStrategy<T>>::accumulate(iter.map(|sv| sv.0))
///     }
/// }
///
/// impl<T> TotalResult<Wrapper<T>> for SaturatingSum
/// where
///     Self: TotalResult<T>,
/// {
///     type TotalRef<'a> = Wrapper<<Self as TotalResult<T>>::TotalRef<'a>>;
///     type Total = Wrapper<<Self as TotalResult<T>>::Total>;
///
///     fn total(state: &Self::State) -> Self::TotalRef<'_> {
///         Wrapper::new(<Self as TotalResult<T>>::total(state))
///     }
///
///     fn into_total(state: Self::State) -> Self::Total {
///         Wrapper::new(<Self as TotalResult<T>>::into_total(state))
///     }
/// }
///
/// impl<T> IndividualResults<Wrapper<T>> for SaturatingSum
/// where
///     Self: IndividualResults<T>,
/// {
///     type Item = Wrapper<<Self as IndividualResults<T>>::Item>;
///
///     fn len(state: &Self::State) -> usize {
///         <Self as IndividualResults<T>>::len(state)
///     }
///
///     fn results<'a>(state: &'a Self::State) -> impl Iterator<Item = &'a Self::Item>
///     where
///         Self::Item: 'a,
///     {
///         <Self as IndividualResults<T>>::results(state).map(Wrapper::ref_cast)
///     }
///
///     fn into_results(state: Self::State) -> impl Iterator<Item = Self::Item> {
///         <Self as IndividualResults<T>>::into_results(state).map(Wrapper::new)
///     }
///
///     fn is_empty(state: &Self::State) -> bool {
///         <Self as IndividualResults<T>>::is_empty(state)
///     }
/// }
///
/// impl<T, Index> IndexResults<Wrapper<T>, Index> for SaturatingSum
/// where
///     Self: IndexResults<T, Index>,
/// {
///     type Output = Wrapper<<Self as IndexResults<T, Index>>::Output>;
///
///     fn get<'a>(state: &'a Self::State, index: Index) -> Option<&'a Self::Output>
///     where
///         Self::Item: 'a,
///     {
///         <Self as IndexResults<T, Index>>::get(state, index).map(Wrapper::ref_cast)
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct SaturatingSum;

macro_rules! impl_accumulate_strategy {
    // (signed $t:ty) => {
    //     impl AccumulateStrategy<$t> for SaturatingSum {
    //         type Error = Infallible;
    //         type State = $t;

    //         fn initialize() -> Self::State {
    //             Self::State::default()
    //         }

    //         fn accumulate_into<I>(state: &mut Self::State, iter: I) -> Result<(), Self::Error>
    //         where
    //             I: Iterator<Item = $t>,
    //         {
    //             for item in iter {
    //                 *state = state.saturating_add(item);
    //             }

    //             Ok(())
    //         }
    //     }

    //     impl TotalResult<$t> for SaturatingSum {
    //         type TotalRef<'a> = $t;
    //         type Total = $t;

    //         fn total(state: &Self::State) -> Self::TotalRef<'_> {
    //             *state
    //         }

    //         fn into_total(state: Self::State) -> Self::Total {
    //             state
    //         }
    //     }
    // };
    // (signedw $t:ty) => {
    //     impl AccumulateStrategy<$t> for SaturatingSum {
    //         type Error = Infallible;
    //         type State = $t;

    //         fn initialize() -> Self::State {
    //             Self::State::default()
    //         }

    //         fn accululate_into<I>(state: &mut Self::State, iter: I) -> Result<(), Self::Error>
    //         where
    //             I: Iterator<Item = $t>,
    //         {
    //             for item in iter {
    //                 state.0 = state.0.saturating_add(item.0);
    //             }

    //             Ok(())
    //         }
    //     }

    //     impl TotalResult<$t> for SaturatingSum {
    //         type TotalRef<'a> = $t;
    //         type Total = $t;

    //         fn total(state: &Self::State) -> Self::TotalRef<'_> {
    //             *state
    //         }

    //         fn into_total(state: Self::State) -> Self::Total {
    //             state
    //         }
    //     }
    // };
    (unsigned $t:ty) => {
        impl AccumulateStrategy<$t> for SaturatingSum {
            type Error = Infallible;
            type State = $t;

            fn initialize() -> Self::State {
                Self::State::default()
            }

            fn accumulate_into<I>(state: &mut Self::State, iter: I) -> Result<(), Self::Error>
            where
                I: Iterator<Item = $t>,
            {
                use std::num::Saturating;
                let Saturating(sum) = iter.map(|x| Saturating(x)).sum();

                *state = state.saturating_add(sum);

                Ok(())
            }
        }

        impl TotalResult<$t> for SaturatingSum {
            type TotalRef<'a> = $t;
            type Total = $t;

            fn total(state: &Self::State) -> Self::TotalRef<'_> {
                *state
            }

            fn into_total(state: Self::State) -> Self::Total {
                state
            }
        }
    };
    // (unsigned2 $t:ty) => {
    //     impl AccumulateStrategy<$t> for SaturatingSum {
    //         type Error = Infallible;
    //         type State = $t;

    //         fn initialize() -> Self::State {
    //             Self::State::default()
    //         }

    //         fn accululate_into<I>(state: &mut Self::State, iter: I) -> Result<(), Self::Error>
    //         where
    //             I: Iterator<Item = $t>,
    //         {
    //             use std::num::Saturating;
    //             let Saturating(sum) = iter.map(|x| Saturating(x.0)).sum();

    //             state.0 = state.0.saturating_add(sum);

    //             Ok(())
    //         }
    //     }

    //     impl TotalResult<$t> for SaturatingSum {
    //         type TotalRef<'a> = $t;
    //         type Total = $t;

    //         fn total(state: &Self::State) -> Self::TotalRef<'_> {
    //             *state
    //         }

    //         fn into_total(state: Self::State) -> Self::Total {
    //             state
    //         }
    //     }
    // };
    ($s: tt $($t: ty),+$(,)?) => {
        $(impl_accumulate_strategy!($s $t);)+
    };
}

// It's not clear what we would want the saturating sum of signed values to be,
// so we'll comment this out for now.
// impl_accumulate_strategy!(signed i8, i16, i32, i64, i128, isize);

impl_accumulate_strategy!(unsigned u8, u16, u32, u64, u128, usize);

// impl_accumulate_strategy!(signedw ScoreValue<i8>, ScoreValue<i16>,
// ScoreValue<i32>, ScoreValue<i64>, ScoreValue<i128>, ScoreValue<isize>);
// impl_accumulate_strategy!(unsigned2 ScoreValue<u8>, ScoreValue<u16>,
// ScoreValue<u32>, ScoreValue<u64>, ScoreValue<u128>, ScoreValue<usize>);
// impl_accumulate_strategy!(signedw ErrorValue<i8>, ErrorValue<i16>,
// ErrorValue<i32>, ErrorValue<i64>, ErrorValue<i128>, ErrorValue<isize>);
// impl_accumulate_strategy!(unsigned2 ErrorValue<u8>, ErrorValue<u16>,
// ErrorValue<u32>, ErrorValue<u64>, ErrorValue<u128>, ErrorValue<usize>);
