use std::marker::PhantomData;

use crate::{
    attributes::Attribute,
    markers::{Init, Uninit},
    MathMl,
};

/// The merror element displays its contents as an ”error message”. The intent of this element is
/// to provide a standard way for programs that generate MathML from other input to report syntax
/// errors in their input.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Error {
    content: MathMl,
    attributes: Vec<Attribute>,
}

impl Error {
    /// Create new `merror` element.
    pub fn with_mathml(math: impl Into<MathMl>) -> Self {
        Self {
            content: math.into(),
            attributes: Default::default(),
        }
    }

    pub fn builder() -> ErrorBuilder<Uninit> {
        ErrorBuilder::default()
    }
}

impl<T> From<T> for Error
where
    T: Into<MathMl>,
{
    fn from(value: T) -> Self {
        Self {
            content: value.into(),
            attributes: Default::default(),
        }
    }
}

crate::tag_from_type!(Error => Error);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ErrorBuilder<T> {
    content: Option<MathMl>,
    attributes: Vec<Attribute>,

    _marker: PhantomData<(T,)>,
}

impl<T> ErrorBuilder<T> {
    pub fn content(self, content: impl Into<MathMl>) -> ErrorBuilder<Init> {
        ErrorBuilder {
            content: Some(content.into()),
            attributes: self.attributes,

            _marker: PhantomData,
        }
    }

    pub fn attr<I, A>(mut self, attr: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        self.attributes.extend(attr.into_iter().map(Into::into));
        self
    }
}

impl ErrorBuilder<Init> {
    pub fn build(self) -> Error {
        Error {
            content: self.content.expect("Content is guaranteed to be init."),
            attributes: self.attributes,
        }
    }
}
