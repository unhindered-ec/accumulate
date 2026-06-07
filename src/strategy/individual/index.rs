use super::IndividualResults;

#[diagnostic::on_unimplemented(
    message = "tried to index results but `{Self}` does not provide indexing for results",
    label = "`IndexResults<{Item}, {Index}>` required here",
    note = "try wrapping your accumulator in an adapter that keeps individual results, such as \
            `KeepResults<{Self}>`"
)]
pub trait IndexResults<Item, Index = usize>: IndividualResults<Item> {
    type Output: ?Sized; // = Self::Item;

    fn get(state: &Self::State, index: Index) -> Option<&Self::Output>;
}
