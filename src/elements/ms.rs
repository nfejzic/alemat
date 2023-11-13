use crate::attributes::Attribute;

/// `ms` element is used to represent "string literals" in expressions meant to be interpreted by
/// computer algebra systems or other systems containing "programming languages".
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StrLiteral {
    num: String,
    attr: Vec<Attribute>,
}

impl<T> From<T> for StrLiteral
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self {
            num: value.into(),
            attr: Default::default(),
        }
    }
}
