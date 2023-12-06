use std::marker::PhantomData;

use crate::{
    attributes::Attribute,
    markers::{Init, Uninit},
    Element, Elements,
};

use super::IntoElements;

/// The merror element displays its contents as an ”error message”. The intent of this element is
/// to provide a standard way for programs that generate MathML from other input to report syntax
/// errors in their input.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Error {
    content: Elements,
    attributes: Vec<Attribute>,
}

impl Error {
    /// Create new `merror` element.
    pub fn with_mathml(math: impl IntoElements) -> Self {
        Self {
            content: math.into_elements(),
            attributes: Default::default(),
        }
    }

    /// Create a builder for [`Error`] element.
    pub fn builder() -> ErrorBuilder<Uninit> {
        ErrorBuilder::default()
    }

    /// Get a reference to the inner content of the [`Error`] element.
    pub fn content(&self) -> &[Element] {
        &self.content
    }

    /// Get a reference to all attributes of the [`Error`] element.
    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

impl From<Element> for Error {
    fn from(value: Element) -> Self {
        Self {
            content: Elements(vec![value]),
            attributes: Default::default(),
        }
    }
}

impl From<Elements> for Error {
    fn from(value: Elements) -> Self {
        Self {
            content: value,
            attributes: Default::default(),
        }
    }
}

crate::element_from_type!(Error => Error);

/// Builder of the [`Error`] element.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ErrorBuilder<T> {
    content: Option<Elements>,
    attributes: Vec<Attribute>,

    _marker: PhantomData<(T,)>,
}

impl<T> ErrorBuilder<T> {
    /// Set the content of the [`Error`] element.
    pub fn content(self, content: impl IntoElements) -> ErrorBuilder<Init> {
        ErrorBuilder {
            content: Some(content.into_elements()),
            attributes: self.attributes,

            _marker: PhantomData,
        }
    }

    /// Add attributes.
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
    /// Build [`Error`] element.
    pub fn build(self) -> Error {
        Error {
            content: self.content.expect("Content is guaranteed to be init."),
            attributes: self.attributes,
        }
    }
}
