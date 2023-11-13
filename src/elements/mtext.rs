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
