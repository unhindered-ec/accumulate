// forward_wrapper_impl!(Wrapper: SaturatingSum);
// forward_wrapper_impl!(ScoreValue: SaturatingSum);

#[macro_export]
macro_rules! forward_wrapper_impl {
    ($wrapper_name: ident : $strategy: ty) => {
        #[automatically_derived]
        impl<T> $crate::strategy::AccumulateStrategy<$wrapper_name<T>> for $strategy
        where
            Self: $crate::strategy::AccumulateStrategy<T>,
        {
            type Error = <Self as $crate::strategy::AccumulateStrategy<T>>::Error;

            type State = <Self as $crate::strategy::AccumulateStrategy<T>>::State;

            fn initialize() -> Self::State {
                <Self as $crate::strategy::AccumulateStrategy<T>>::initialize()
            }

            fn accumulate_into<I>(state: &mut Self::State, iter: I) -> Result<(), Self::Error>
            where
                I: Iterator<Item = $wrapper_name<T>>,
            {
                <Self as $crate::strategy::AccumulateStrategy<T>>::accumulate_into(
                    state,
                    iter.map(|sv| sv.0),
                )
            }

            fn accumulate<I>(iter: I) -> Result<Self::State, Self::Error>
            where
                I: Iterator<Item = $wrapper_name<T>>,
            {
                <Self as $crate::strategy::AccumulateStrategy<T>>::accumulate(iter.map(|sv| sv.0))
            }
        }

        #[automatically_derived]
        impl<T> $crate::total::TotalResult<$wrapper_name<T>> for $strategy
        where
            Self: $crate::total::TotalResult<T>,
        {
            type TotalRef<'a> =
                $wrapper_name<<Self as $crate::total::TotalResult<T>>::TotalRef<'a>>;

            type Total = $wrapper_name<<Self as $crate::total::TotalResult<T>>::Total>;

            fn total(state: &Self::State) -> Self::TotalRef<'_> {
                $wrapper_name::new(<Self as $crate::total::TotalResult<T>>::total(state))
            }

            fn into_total(state: Self::State) -> Self::Total {
                $wrapper_name::new(<Self as $crate::total::TotalResult<T>>::into_total(state))
            }
        }

        #[automatically_derived]
        impl<T> $crate::results::IndividualResults<$wrapper_name<T>> for $strategy
        where
            Self: $crate::results::IndividualResults<T>,
            for<'a> <Self as $crate::results::IndividualResults<T>>::Item: 'a,
        {
            type Item = $wrapper_name<<Self as $crate::results::IndividualResults<T>>::Item>;

            fn len(state: &Self::State) -> usize {
                <Self as $crate::results::IndividualResults<T>>::len(state)
            }

            fn results<'a>(state: &'a Self::State) -> impl Iterator<Item = &'a Self::Item>
            where
                Self::Item: 'a,
            {
                <Self as $crate::results::IndividualResults<T>>::results(state)
                    .map($wrapper_name::ref_cast)
            }

            fn into_results(state: Self::State) -> impl Iterator<Item = Self::Item> {
                <Self as $crate::results::IndividualResults<T>>::into_results(state)
                    .map($wrapper_name::new)
            }

            fn is_empty(state: &Self::State) -> bool {
                <Self as $crate::results::IndividualResults<T>>::is_empty(state)
            }
        }

        #[automatically_derived]
        impl<T, Index> $crate::results::IndexResults<$wrapper_name<T>, Index> for $strategy
        where
            Self: $crate::results::IndexResults<T, Index>,
            <Self as $crate::results::IndividualResults<T>>::Item: 'static,
        {
            type Output = $wrapper_name<<Self as $crate::results::IndexResults<T, Index>>::Output>;

            fn get<'a>(state: &'a Self::State, index: Index) -> Option<&'a Self::Output>
            where
                Self::Item: 'a,
            {
                <Self as $crate::results::IndexResults<T, Index>>::get(state, index)
                    .map($wrapper_name::ref_cast)
            }
        }
    };
}
