use std::marker::PhantomData;

use crate::{
    attributes::Attribute,
    markers::{Init, Uninit},
    MathMl, Tag,
};

/// An attribute of `mi` element. Either one of the global [`Attribute`]s, or `mathvariant`
/// attribute.
///
/// [`Attribute`]: crate::attributes::Attribute
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IdentAttr {
    /// Global attribute.
    Global(Attribute),

    /// The linethickness attribute indicates the fraction line thickness to use for the fraction
    /// bar.
    /// It must have a value that is a valid
    /// [<length-percentage>](https://www.w3.org/TR/css-values-4/#typedef-length-percentage).
    MathVariant(String),
}

/// The `mi` (`Ident`) element represents a symbolic name or arbitrary text that should be rendered
/// as an identifier. Identifiers can include variables, function names, and symbolic constants.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ident {
    ident: String,
    attributes: Vec<IdentAttr>,
}

impl Ident {
    pub fn builder() -> IdentBuilder<Uninit> {
        IdentBuilder::default()
    }
}

impl<T> From<T> for Ident
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self {
            ident: value.into(),
            attributes: Default::default(),
        }
    }
}

impl From<Ident> for MathMl {
    fn from(value: Ident) -> Self {
        MathMl {
            content: vec![Tag::Ident(value)],
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IdentBuilder<T> {
    ident: Option<String>,
    attributes: Vec<IdentAttr>,
    _marker: PhantomData<(T,)>,
}

impl<T> IdentBuilder<T> {
    pub fn ident(self, ident: impl Into<String>) -> IdentBuilder<Uninit> {
        IdentBuilder {
            ident: Some(ident.into()),
            attributes: self.attributes,
            _marker: PhantomData,
        }
    }

    pub fn attr<I, A>(mut self, attr: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<IdentAttr>,
    {
        self.attributes.extend(attr.into_iter().map(Into::into));
        self
    }
}

impl IdentBuilder<Init> {
    pub fn build(self) -> Ident {
        Ident {
            ident: self.ident.expect("Content is guaranteed to be init."),
            attributes: self.attributes,
        }
    }
}
