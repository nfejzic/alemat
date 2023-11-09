use crate::{attributes::Attribute, MathMl};

/// The display attribute, if present, must be an ASCII case-insensitive match to `block` or
/// `inline`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DisplayAttr {
    /// `block` display attribute.
    Block,

    /// `inline` display attribute.
    Inline,
}

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

/// MathML specifies a single top-level or root math element, which encapsulates each instance of
/// MathML markup within a document. All other MathML content must be contained in a <math>
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
