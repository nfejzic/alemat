use crate::{attributes::Attribute, Element, Elements};

use super::IntoElements;

/// The `mpadded` element accepts global attributes as well as the `width`, `height`, `depth`,
/// `lspace` and `voffset`
///
/// The `width`, `height`, `depth`, `lspace` and `voffset` if present, must have a value that is a
/// valid [`<length-percentage>`](https://www.w3.org/TR/css-values-4/#typedef-length-percentage).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PaddedAttr {
    /// A `<length-percentage>` indicating the desired horizontal length of the `mpadded` element.
    Width(String),

    /// A `<length-percentage>` indicating the desired height (above the baseline) of the `mpadded`
    /// element.
    Height(String),

    /// A `<length-percentage>` indicating the desired depth (below the baseline) of the `mpadded`
    /// element.
    Depth(String),

    /// A `<length-percentage>` indicating the horizontal location of the positioning point of the
    /// child content with respect to the positioning point of the `mpadded` element.
    LeftSpace(String),

    /// A `<length-percentage>` indicating the vertical location of the positioning point of the
    /// child content with respect to the positioning point of the `mpadded` element.
    VerticalOffset(String),

    /// One of the global [`Attribute`]s.
    Global(Attribute),
}

impl From<Attribute> for PaddedAttr {
    fn from(value: Attribute) -> Self {
        Self::Global(value)
    }
}

/// The `mpadded` element renders the same as its in-flow child content, but with the size and
/// relative positioning point of its content modified according to `mpadded`’s attributes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Padded {
    children: Elements,
    attributes: Vec<PaddedAttr>,
}

impl From<Elements> for Padded {
    fn from(value: Elements) -> Self {
        Self {
            children: value,
            attributes: Default::default(),
        }
    }
}

impl<const N: usize, I> From<[I; N]> for Padded
where
    I: Into<Element>,
{
    fn from(value: [I; N]) -> Self {
        Self {
            children: value.into_elements(),
            attributes: Default::default(),
        }
    }
}

impl Padded {
    /// Add attributes.
    pub fn add_attr<I, A>(&mut self, attr: I)
    where
        I: IntoIterator<Item = A>,
        A: Into<PaddedAttr>,
    {
        self.attributes.extend(attr.into_iter().map(Into::into));
    }

    /// Create new instance of [`Padded`] with additional attributes.
    pub fn with_attr<I, A>(mut self, attr: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<PaddedAttr>,
    {
        self.attributes.extend(attr.into_iter().map(Into::into));
        self
    }

    /// Get a reference to the children of the [`Padded`] element.
    pub fn children(&self) -> &[Element] {
        &self.children
    }

    /// Get a reference to all attributes of the [`Padded`] element.
    pub fn attributes(&self) -> &[PaddedAttr] {
        &self.attributes
    }
}

crate::element_from_type!(Padded => Padded);
