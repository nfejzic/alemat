use crate::attributes::Attribute;

/// `ms` element is used to represent "string literals" in expressions meant to be interpreted by
/// computer algebra systems or other systems containing "programming languages".
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StrLiteral {
    literal: String,
    attr: Vec<Attribute>,
}

impl<T> From<T> for StrLiteral
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self {
            literal: value.into(),
            attr: Default::default(),
        }
    }
}

impl StrLiteral {
    pub fn add_attr<I, A>(mut self, attr: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        self.attr.extend(attr.into_iter().map(Into::into));
        self
    }
}
