use crate::{attributes::Attribute, Element, Elements};

use super::IntoElements;

/// The `mphantom` element was introduced to render its content invisibly, but with the same
/// metrics size and other dimensions, including alphabetic baseline position that its contents
/// would have if they were rendered normally.
///
/// The user agent stylesheet must contain the following rule in order to hide the content:
///
/// ```css
/// mphantom {
///   visibility: hidden;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Phantom {
    children: Elements,
    attributes: Vec<Attribute>,
}

impl From<Elements> for Phantom {
    fn from(value: Elements) -> Self {
        Self {
            children: value,
            attributes: Default::default(),
        }
    }
}

impl<const N: usize, I: Into<Element>> From<[I; N]> for Phantom {
    fn from(value: [I; N]) -> Self {
        Self {
            children: value.into_elements(),
            attributes: Default::default(),
        }
    }
}

impl Phantom {
    pub fn add_attr<I, A>(&mut self, attr: I)
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        self.attributes.extend(attr.into_iter().map(Into::into));
    }

    pub fn with_attr<I, A>(mut self, attr: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        self.attributes.extend(attr.into_iter().map(Into::into));
        self
    }

    pub fn children(&self) -> &[Element] {
        &self.children
    }

    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

crate::element_from_type!(Phantom => Phantom);
