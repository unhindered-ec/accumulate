pub trait Wrapper {
    type Contained;

    fn wrap(contained: Self::Contained) -> Self;
    fn wrap_ref(contained: &Self::Contained) -> &Self;
    fn wrap_mut(contained: &mut Self::Contained) -> &mut Self;

    fn unwrap(self) -> Self::Contained;
    fn unwrap_ref(&self) -> &Self::Contained;
    fn unwrap_mut(&mut self) -> &mut Self::Contained;
}
