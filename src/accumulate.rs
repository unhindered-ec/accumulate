use super::{accumulated::Accumulated, strategy::AccumulateStrategy};

pub trait Accumulate: Iterator {
    /// # Errors
    /// Returns an `Acc::Error` if there is an error while accumulating.
    fn accumulate<Acc>(&mut self) -> Result<Accumulated<Self::Item, Acc>, Acc::Error>
    where
        Acc: AccumulateStrategy<Self::Item>;
}

impl<I: Iterator + ?Sized> Accumulate for I {
    fn accumulate<Acc>(&mut self) -> Result<Accumulated<Self::Item, Acc>, Acc::Error>
    where
        Acc: AccumulateStrategy<Self::Item>,
    {
        match Acc::accumulate(self) {
            Ok(state) => Ok(Accumulated::from_state(state)),
            Err(e) => Err(e),
        }
    }
}

// #[cfg(test)]
// mod test {
//     use crate::{accumulate::Accumulate, accumulated::Accumulated};

//     #[test]
//     fn test() {
//         let values = [1, 2, 3];
//         let accumulated: Accumulated<i32> =
// values.into_iter().accumulate().unwrap();     }
// }
