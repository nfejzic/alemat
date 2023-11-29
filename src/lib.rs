//! Library for type-safe building of MathML.

pub mod attributes;
mod default_renderer;
pub mod elements;
pub mod markers;
mod to_mathml;

pub use default_renderer::MathMlFormatter;
pub(crate) use elements::element_from_type;
pub use elements::{Element, Elements};
pub use to_mathml::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MathMl {
    content: Vec<Element>,
    // TODO: decide what fields should go inside
}

impl MathMl {
    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

macro_rules! from_types {
    ($($type:path),* $(,)? => $for_type:path; $func:expr) => {
        $(
        impl From<$type> for $for_type {
            fn from(value: $type) -> Self {
                let from_value = $func;
                from_value(value)
            }
        }
        )*
    };
}

pub(crate) use from_types;
