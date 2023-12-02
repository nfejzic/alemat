use crate::{attributes::Attribute, Element, Elements};

/// The `mpadded` element accepts global attributes as well as the `width`, `height`, `depth`,
/// `lspace` and `voffset`
///
/// The `width`, `height`, `depth`, `lspace` and `voffset` if present, must have a value that is a
/// valid [`<length-percentage>`](https://www.w3.org/TR/css-values-4/#typedef-length-percentage).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PaddedAttr {
    Width(String),
    Height(String),
    Depth(String),
    LeftSpace(String),
    VerticalOffset(String),

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
            children: Elements(value.map(Into::into).to_vec()),
            attributes: Default::default(),
        }
    }
}

impl Padded {
    pub fn add_attr<I, A>(&mut self, attr: I)
    where
        I: IntoIterator<Item = A>,
        A: Into<PaddedAttr>,
    {
        self.attributes.extend(attr.into_iter().map(Into::into));
    }

    pub fn with_attr<I, A>(mut self, attr: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<PaddedAttr>,
    {
        self.attributes.extend(attr.into_iter().map(Into::into));
        self
    }

    pub fn children(&self) -> &[Element] {
        &self.children
    }

    pub fn attributes(&self) -> &[PaddedAttr] {
        &self.attributes
    }
}

crate::element_from_type!(Padded => Padded);
