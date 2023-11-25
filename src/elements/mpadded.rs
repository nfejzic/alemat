use crate::{attributes::Attribute, MathMl};

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
    children: MathMl,
    attributes: Vec<PaddedAttr>,
}

impl<T> From<T> for Padded
where
    T: Into<MathMl>,
{
    fn from(value: T) -> Self {
        Self {
            children: value.into(),
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
}

crate::tag_from_type!(Padded => Padded);
