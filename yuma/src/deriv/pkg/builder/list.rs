use super::{AsBuilder, PkgBuilder};

/// A wrapper trait that allows that add meathod to take both arrays and single builders
pub trait AsPkgBuilderList {
    fn list(self) -> Vec<PkgBuilder>;
}

impl AsPkgBuilderList for PkgBuilder {
    fn list(self) -> Vec<PkgBuilder> {
        vec![self]
    }
}

impl AsPkgBuilderList for &str {
    fn list(self) -> Vec<PkgBuilder> {
        vec![self.into()]
    }
}

impl<const N: usize> AsPkgBuilderList for [PkgBuilder; N] {
    fn list(self) -> Vec<PkgBuilder> {
        self.into_iter().collect()
    }
}

impl<const N: usize> AsPkgBuilderList for [&str; N] {
    fn list(self) -> Vec<PkgBuilder> {
        vec![self.as_slice().into()]
    }
}

impl<E1, E2> AsPkgBuilderList for (E1, E2)
where
    E1: AsBuilder,
{
    fn list(self) -> Vec<PkgBuilder> {
        todo!()
    }
}
