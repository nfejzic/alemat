//! Library for type-safe building of MathML.

pub mod attributes;
pub mod elements;
pub mod markers;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Tag {
    Num(Num),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MathMl {
    content: Vec<Tag>,
    // TODO: decide what fields should go inside
}

macro_rules! from_types {
    ($($type:ty),* $(,)?) => {
        $(
        impl From<$type> for Num {
            fn from(value: $type) -> Self {
                Self {
                    num: format!("{}", value),
                    attributes: Vec::default(),
                }
            }
        }
        )*
    };
}

use elements::Num;
pub(crate) use from_types;
