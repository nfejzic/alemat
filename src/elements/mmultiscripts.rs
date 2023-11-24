use std::marker::PhantomData;

use crate::{
    attributes::Attribute,
    markers::{Init, Uninit},
    MathMl,
};

/// Presubscripts and tensor notations are represented by the `mmultiscripts` element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Multiscripts {
    content: MathMl,
    /// Accepts the global [`Attribute`]s.
    attributes: Vec<Attribute>,
}

impl<T> From<T> for Multiscripts
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

impl Multiscripts {
    pub fn builder() -> MultiscriptsBuilder<Uninit> {
        MultiscriptsBuilder::default()
    }
}

crate::tag_from_type!(Multiscripts => Multiscripts);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MultiscriptsBuilder<T> {
    content: Option<MathMl>,
    attr: Vec<Attribute>,

    _marker: PhantomData<(T,)>,
}

impl<T> MultiscriptsBuilder<T> {
    pub fn content(self, content: impl Into<MathMl>) -> MultiscriptsBuilder<Init> {
        MultiscriptsBuilder {
            content: Some(content.into()),
            attr: self.attr,
            _marker: PhantomData,
        }
    }

    pub fn attr<I, A>(mut self, attr: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        self.attr.extend(attr.into_iter().map(Into::into));
        self
    }
}

impl MultiscriptsBuilder<Init> {
    pub fn build(self) -> Multiscripts {
        Multiscripts {
            content: self.content.expect("Guaranteed to be init"),
            attributes: self.attr,
        }
    }
}

/// Presubscripts and tensor notations are represented by the `mmultiscripts` element. The
/// `mprescripts` element is used as a separator between the `postscripts` and `prescripts`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Prescripts {
    /// Accepts the global [`Attribute`]s.
    attr: Vec<Attribute>,
}

impl Prescripts {
    pub fn with_attr<I, A>(attr: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        Self {
            attr: attr.into_iter().map(Into::into).collect(),
        }
    }
}
