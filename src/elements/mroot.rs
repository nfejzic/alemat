use std::marker::PhantomData;

use crate::{
    attributes::Attribute,
    markers::{Init, Uninit},
    MathMl,
};

/// The radical elements construct an expression with a root symbol `√` with a line over the content.
/// The msqrt element is used for square roots, while the mroot element is used to draw radicals
/// with indices, e.g. a cube root.
///
/// The `msqrt` and `mroot` elements accept the global [`Attribute`]s.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Radicals {
    /// The index of the radical, e.g. the 3 in `∛`.
    index: String,
    content: MathMl,
    attributes: Vec<Attribute>,
}

impl Radicals {
    pub fn builder() -> RadicalsBuilder<Uninit, Uninit> {
        RadicalsBuilder::default()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RadicalsBuilder<T1, T2> {
    index: Option<String>,
    content: Option<MathMl>,
    attr: Vec<Attribute>,

    _marker: PhantomData<(T1, T2)>,
}

impl<T1, T2> RadicalsBuilder<T1, T2> {
    pub fn index(self, index: impl Into<String>) -> RadicalsBuilder<Init, T2> {
        RadicalsBuilder {
            index: Some(index.into()),
            content: self.content,
            attr: self.attr,
            _marker: PhantomData,
        }
    }

    pub fn content(self, content: impl Into<MathMl>) -> RadicalsBuilder<T1, Init> {
        RadicalsBuilder {
            index: self.index,
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

impl RadicalsBuilder<Init, Init> {
    pub fn build(self) -> Radicals {
        Radicals {
            index: self.index.expect("Index is guaranteed to be init."),
            content: self.content.expect("Content is guaranteed to be init."),
            attributes: self.attr,
        }
    }
}
