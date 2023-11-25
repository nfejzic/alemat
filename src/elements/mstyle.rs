use crate::{attributes::Attribute, MathMl};

/// `mstyle` element was introduced to make style changes that affect the rendering of its contents.
///
/// The `mstyle` element accepts the global [`Attribute`]s.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Style {
    children: MathMl,
    attr: Vec<Attribute>,
}

impl<T> From<T> for Style
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

impl Style {
    pub fn add_attr<I, A>(&mut self, attr: I)
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        self.attr.extend(attr.into_iter().map(Into::into));
    }
}

crate::tag_from_type!(Style => Style);
