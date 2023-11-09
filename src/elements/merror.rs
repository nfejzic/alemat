use crate::{attributes::Attribute, MathMl};

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
