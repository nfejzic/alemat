use crate::{attributes::Attribute, MathMl};

/// The `mphantom` element was introduced to render its content invisibly, but with the same
/// metrics size and other dimensions, including alphabetic baseline position that its contents
/// would have if they were rendered normally.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Phantom {
    children: MathMl,
    attributes: Vec<Attribute>,
}

impl<T> From<T> for Phantom
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
