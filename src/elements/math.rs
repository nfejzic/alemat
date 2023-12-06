use std::marker::PhantomData;

use crate::{
    attributes::Attribute,
    markers::{Init, Uninit},
    DisplayAttr, MathMl,
};

/// An attribute of `math` element. Either one of the global [`Attribute`]s, `display` or `alttext`
/// attribute.
///
/// [`Attribute`]: crate::attributes::Attribute
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MathAttr {
    Global(Attribute),
    Display(DisplayAttr),
    AltText(String),
}

// TODO: figure out if this element is actually needed.
/// MathML specifies a single top-level or root math element, which encapsulates each instance of
/// MathML markup within a document. All other MathML content must be contained in a `<math>`
/// element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Math {
    content: MathMl,
    attributes: Vec<MathAttr>,
}

impl Math {
    /// Create new `math` element.
    pub fn with_mathml(math: impl Into<MathMl>) -> Self {
        Self {
            content: math.into(),
            attributes: Default::default(),
        }
    }

    pub fn builder() -> MathBuilder<Uninit> {
        MathBuilder::default()
    }
}

impl<T> From<T> for Math
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

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MathBuilder<T> {
    content: Option<MathMl>,
    attributes: Vec<MathAttr>,

    _marker: PhantomData<(T,)>,
}

impl<T> MathBuilder<T> {
    pub fn content(self, content: impl Into<MathMl>) -> MathBuilder<Init> {
        MathBuilder {
            content: Some(content.into()),
            attributes: self.attributes,

            _marker: PhantomData,
        }
    }

    pub fn attr<I, A>(mut self, attr: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<MathAttr>,
    {
        self.attributes.extend(attr.into_iter().map(Into::into));
        self
    }
}

impl MathBuilder<Init> {
    pub fn build(self) -> Math {
        Math {
            content: self.content.expect("Content is guaranteed to be init."),
            attributes: self.attributes,
        }
    }
}
