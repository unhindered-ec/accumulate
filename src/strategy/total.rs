use super::AccumulateStrategy;

#[diagnostic::on_unimplemented(
    message = "tried to access total result but `{Self}` does not provide a total result",
    label = "`TotalResult<{Item}>` required here",
    note = "try wrapping your accumulator with an accumulator that calculates a total result,  \
            such as `SaturatingSum`"
)]
pub trait TotalResult<Item>: AccumulateStrategy<Item> {
    type Total;

    fn total(state: &Self::State) -> &Self::Total;
    fn into_total(state: Self::State) -> Self::Total;
}
