mod dict;

use std::marker::PhantomData;

use crate::{
    attributes::Attribute,
    markers::{Init, Uninit},
};

/// The `mi` (`Ident`) element represents a symbolic name or arbitrary text that should be rendered
/// as an identifier. Identifiers can include variables, function names, and symbolic constants.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ident {
    ident: String,
    attributes: Vec<Attribute>,
}

impl Ident {
    pub fn builder() -> IdentBuilder<Uninit> {
        IdentBuilder::default()
    }

    pub fn ident(&self) -> &str {
        &self.ident
    }

    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
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

crate::element_from_type!(Ident => Ident);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IdentBuilder<T> {
    ident: Option<String>,
    attributes: Vec<Attribute>,
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
        A: Into<Attribute>,
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
