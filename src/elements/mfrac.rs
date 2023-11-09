use crate::{attributes::Attribute, MathMl};

/// An attribute of `mfrac` element. Either one of the global [`Attribute`]s, or `linethickness`
/// attribute.
///
/// [`Attribute`]: crate::attributes::Attribute
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FracAttr {
    /// Global attribute.
    Global(Attribute),

    /// The linethickness attribute indicates the fraction line thickness to use for the fraction
    /// bar.
    /// It must have a value that is a valid
    /// [<length-percentage>](https://www.w3.org/TR/css-values-4/#typedef-length-percentage).
    LineThickness(String),
}

/// The merror element displays its contents as an ”error message”. The intent of this element is
/// to provide a standard way for programs that generate MathML from other input to report syntax
/// errors in their input.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frac {
    content: MathMl,
    attributes: Vec<FracAttr>,
}

impl Frac {
    /// Create new `mfrac` element.
    pub fn with_mathml(math: impl Into<MathMl>) -> Self {
        Self {
            content: math.into(),
            attributes: Default::default(),
        }
    }
}

impl<T> From<T> for Frac
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
