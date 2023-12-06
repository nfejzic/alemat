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
    /// Add attributes.
    pub fn add_attr<I, A>(&mut self, attr: I)
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        self.attr.extend(attr.into_iter().map(Into::into));
    }

    /// Create a new instance of [`StrLiteral`] with additional attributes.
    pub fn with_attr<I, A>(mut self, attr: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        self.attr.extend(attr.into_iter().map(Into::into));
        self
    }

    /// Get a reference to the literal content of the [`StrLiteral`] element.
    pub fn content(&self) -> &str {
        &self.literal
    }

    /// Get a reference to all attributes of the [`StrLiteral`] element.
    pub fn attributes(&self) -> &[Attribute] {
        &self.attr
    }
}

crate::element_from_type!(StrLiteral => StrLiteral);
