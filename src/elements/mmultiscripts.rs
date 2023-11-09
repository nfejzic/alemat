use crate::{attributes::Attribute, MathMl};

/// Presubscripts and tensor notations are represented by the `mmultiscripts` element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Multiscripts {
    content: MathMl,
    attributes: Vec<Attribute>,
}

impl<T> From<T> for Multiscripts
where
    T: Into<MathMl>,
{
    fn from(value: T) -> Self {
        Self {
            content: value.into(),
            attributes: Default::default(),
        }
    }
}
