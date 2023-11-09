use crate::attributes::Attribute;

/// The mn element represents a "numeric literal" or other data that should be rendered as a
/// numeric literal. Generally speaking, a numeric literal is a sequence of digits, perhaps
/// including a decimal point, representing an unsigned integer or real number.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Num {
    num: String,
    attributes: Vec<Attribute>,
}

impl<T> From<T> for Num
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self {
            num: value.into(),
            attributes: Default::default(),
        }
    }
}
