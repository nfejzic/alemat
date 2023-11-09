use crate::attributes::Attribute;

/// An attribute of `mi` element. Either one of the global [`Attribute`]s, or `mathvariant`
/// attribute.
///
/// [`Attribute`]: crate::attributes::Attribute
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IdentAttr {
    /// Global attribute.
    Global(Attribute),

    /// The linethickness attribute indicates the fraction line thickness to use for the fraction
    /// bar.
    /// It must have a value that is a valid
    /// [<length-percentage>](https://www.w3.org/TR/css-values-4/#typedef-length-percentage).
    MathVariant,
}

/// The `mi` (`Ident`) element represents a symbolic name or arbitrary text that should be rendered
/// as an identifier. Identifiers can include variables, function names, and symbolic constants.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ident {
    ident: String,
    attributes: Vec<IdentAttr>,
}

impl<T> From<T> for Ident
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self {
            ident: value.into(),
            attributes: Default::default(),
        }
    }
}
