//! Provides various strategies for
//! accumulating a collection of values into a single value.
//!
//! There are two types of accumulator
//! strategies defined in the `accumulate` package:
//!
//! - "Base" accumulators, which specify the specific accumulation strategy
//! - "Adapters", which modify the behavior of the accumulators they "wrap"
//!   (which can be either base accumulators or previously wrapped accumulators)
//!
//! ## Base accumulators
//!
//! The current base accumulators include:
//!
//! - [`Sum`](sum::Sum) adds up the values. This is auto implemented for all
//!   types `T: AddAssign + Sum + Default`
//! - [`WrappingSum`](wrapping_sum::WrappingSum) adds the values with wrapping.
//!   This is auto implemented for all types `Wrapping<T>: AddAssign + Sum` and
//!   `T: Default`
//! - [`SaturatingSum`](saturating_sum::SaturatingSum) adds the values with
//!   saturation. For technical reasons, this must be implemented separately for
//!   each type.
//! - [`StoreResults`](keep_results::StoreResults`) does not add up the values,
//!   but instead stores them in a `Vec` for later access. If you want to store
//!   _and_ sum you can use [`Combine`](combine::Combine) or the helper type
//!   [`KeepResults`](keep_results::KeepResults).
//!
//! For several of these we require `T: Default`. This is provide a default
//! value to return if the collection is empty, so you need to make sure that
//! the default value is compatible with that usage (i.e., it's a _zero_ of
//! summation on the type `T`).
//!
//! > ℹ️ **Note:**
//! > There are generic implementations for all of these except `SaturatingSum`.
//! > Because we can't currently
//! > specify the necessary features in the type, if you want saturating
//! > accumulation for a new item type you will
//! > have to implement a number of traits.
//!
//! TODO: I feel like the note above is fairly vague, especially when
//! moved away from the `ErrorValue` and `ScoreValue` examples in
//! `unhindered-ec`.
//!
//! ## Adapters
//!
//! The current adapters include:
//!
//! - [`Widen<T, Strategy>`](widen::Widen) first converts the values to type `T`
//!   (which is presumably a "wider" type), and then uses a provided
//!   [`AccumulateStrategy`](strategy::AccumulateStrategy) to accumulate these
//!   widened values
//! - [`Combine<IndividualStrategy, TotalStrategy>`](combine::Combine) allows
//!   you to use different strategies to (a) combine the individuals and (b)
//!   create the total.
//!
//! [`KeepResults<T>`](keep_results::KeepResults<T>) is an alias for
//! [`Combine<StoreResults, T>`](combine::Combine). This allows you
//! to store the individual results, while also combining them into a final
//! aggregate value using the strategy `T` (e.g., `Sum` to add them up).
//!
//! TODO: Add an example or two here the illustrate some of the key concepts.

pub mod accumulate;
pub mod accumulated;
pub mod default;
pub mod results;
pub mod strategy;
pub mod total;

pub mod combine;
pub mod keep_results;
pub mod widen;

pub mod saturating_sum;
pub mod sum;
pub mod wrapping_sum;

pub mod wrapper_item;
