mod index;
pub use index::IndexResults;

use crate::strategy::AccumulateStrategy;

#[diagnostic::on_unimplemented(
    message = "tried to access individual results but `{Self}` does not provide individual results",
    label = "`IndividualResults<{Item}>` required here",
    note = "try wrapping your accumulator in an adapter that keeps individual results, such as \
            `KeepResults<{Self}>`"
)]
pub trait IndividualResults<Item>: AccumulateStrategy<Item> {
    type Item;

    fn len(state: &Self::State) -> usize;

    fn results<'a>(state: &'a Self::State) -> impl Iterator<Item = &'a Self::Item>
    where
        Self::Item: 'a;

    fn into_results(state: Self::State) -> impl Iterator<Item = Self::Item>;

    fn is_empty(state: &Self::State) -> bool {
        Self::len(state) == 0
    }
}
