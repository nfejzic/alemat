use crate::{attributes::Attribute, MathMl};

/// The `mrow` element is used to group together any number of sub-expressions, usually consisting
/// of one or more `mo` elements acting as "operators" on one or more other expressions that are
/// their "operands".
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Row {
    children: MathMl,
    attr: Vec<Attribute>,
}

impl<T> From<T> for Row
where
    T: Into<MathMl>,
{
    fn from(value: T) -> Self {
        Self {
            children: value.into(),
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
}

crate::tag_from_type!(Row => Row);
