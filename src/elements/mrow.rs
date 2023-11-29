use crate::{attributes::Attribute, Element, Elements};

/// The `mrow` element is used to group together any number of sub-expressions, usually consisting
/// of one or more `mo` elements acting as "operators" on one or more other expressions that are
/// their "operands".
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Row {
    children: Elements,
    attr: Vec<Attribute>,
}

impl From<Elements> for Row {
    fn from(value: Elements) -> Self {
        Self {
            children: value,
            attr: Default::default(),
        }
    }
}

impl Row {
    pub fn add_attr<I, A>(&mut self, attr: I)
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        self.attr.extend(attr.into_iter().map(Into::into));
    }

    pub fn children(&self) -> &[Element] {
        &self.children
    }

    pub fn attributes(&self) -> &[Attribute] {
        &self.attr
    }
}

crate::element_from_type!(Row => Row);

/// Create a `mrow` of [`Element`]s.
///
/// # Example:
///
/// ```rust
/// use alemat::{row, Element};
/// use alemat::elements::{Ident, Num};
/// use alemat::elements::grouping::Row;
///
/// let row: Row = row![Ident::from("x"), Num::from(42)];
/// ```
#[macro_export]
macro_rules! row {
    ($($element:expr),* $(,)?) => {{
        use $crate::elements::IntoElements;
        let children = [$($crate::elements::Element::from($element)),*].into_elements();

        $crate::elements::grouping::Row::from(children)
    }}
}
