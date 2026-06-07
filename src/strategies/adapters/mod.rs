mod combine;
mod widen;

pub use combine::Combine;
pub use widen::Widen;

use crate::strategies::store_results::StoreResults;
pub type KeepResults<T> = Combine<StoreResults, T>;
