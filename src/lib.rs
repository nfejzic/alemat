//! Library for type-safe building of MathML.

pub mod attributes;
pub mod elements;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Tag {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MathMl {
    content: Vec<Tag>,
    // TODO: decide what fields should go inside
}
