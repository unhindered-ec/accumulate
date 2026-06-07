#[diagnostic::on_unimplemented(
    message = "No default accumulation strategy specified for type {Self}",
    label = "explicit accumulation strategy required here",
    note = "If you are trying to use Accumulate<{Self}>, use Accumulate<{Self}, MyStrategy> \
            instead,\nwhere MyStrategy specifies the strategy of accumulation."
)]
pub trait DefaultAccumulateStrategy {
    type Strategy;
}

#[doc(hidden)]
#[macro_export]
macro_rules! default_to {
    ($t: ty => $d: ty) => {
        impl $crate::strategy::DefaultAccumulateStrategy for $t {
            type Strategy = $d;
        }
    };
    ($($t: ty => $d: ty),+$(,)?) => {
        $($crate::strategy::default_to!($t => $d);)+
    }
}

pub use default_to;
