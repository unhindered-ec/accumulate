pub trait AccumulateStrategy<Item>
where
    Self: Sized,
{
    type Error;
    type State;

    fn initialize() -> Self::State;

    /// # Errors
    /// Returns a `Self::Error` if there is an error during accumulation.
    fn accumulate_into<I>(state: &mut Self::State, iter: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = Item>;

    /// # Errors
    /// Returns a `Self::Error` if there is an error during accumulation.
    fn accumulate<I>(iter: I) -> Result<Self::State, Self::Error>
    where
        I: Iterator<Item = Item>,
    {
        let mut state = Self::initialize();
        Self::accumulate_into(&mut state, iter)?;
        Ok(state)
    }
}
