use crate::attributes::Attribute;

/// The `mtext` element is used to represent arbitrary text that should be rendered as itself. In
/// general, the `mtext` element is intended to denote commentary text.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Text {
    /// The text to be rendered.
    text: String,

    /// The `mtext` element accepts the global [`Attribute`]s.
    attr: Vec<Attribute>,
}

impl<T> From<T> for Text
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self {
            text: value.into(),
            attr: Default::default(),
        }
    }
}

impl Text {
    /// Add attributes.
    pub fn add_attr<I, A>(&mut self, attr: I)
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        self.attr.extend(attr.into_iter().map(Into::into));
    }

    /// Create a new instance of [`Text`] with additional attributes.
    pub fn with_attr<I, A>(mut self, attr: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        self.attr.extend(attr.into_iter().map(Into::into));
        self
    }

    /// Get a reference to the text content of the [`Text`] element.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get a reference to all attributes of the [`Text`] element.
    pub fn attributes(&self) -> &[Attribute] {
        &self.attr
    }
}

crate::element_from_type!(Text => Text);
