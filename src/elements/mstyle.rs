use crate::{attributes::Attribute, Element, Elements};

use super::IntoElements;

/// `mstyle` element was introduced to make style changes that affect the rendering of its
/// contents.
///
/// The `mstyle` element accepts the global [`Attribute`]s.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Style {
    children: Elements,
    attr: Vec<Attribute>,
}

impl From<Elements> for Style {
    fn from(value: Elements) -> Self {
        Self {
            children: value,
            attr: Default::default(),
        }
    }
}

impl<const N: usize, I: Into<Element>> From<[I; N]> for Style {
    fn from(value: [I; N]) -> Self {
        Self {
            children: value.into_elements(),
            attr: Vec::default(),
        }
    }
}

impl Style {
    pub fn add_attr<I, A>(&mut self, attr: I)
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        self.attr.extend(attr.into_iter().map(Into::into));
    }

    pub fn with_attr<I, A>(mut self, attr: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        self.attr.extend(attr.into_iter().map(Into::into));
        self
    }

    pub fn children(&self) -> &[Element] {
        &self.children
    }

    pub fn attributes(&self) -> &[Attribute] {
        &self.attr
    }
}

crate::element_from_type!(Style => Style);
