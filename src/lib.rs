//! Library for type-safe building of MathML.

pub mod attributes;
pub mod elements;
pub mod markers;

use elements::{
    Annotation, Frac, Ident, Matrix, Num, Operator, Padded, Semantics, Space, Table, Text,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Tag {
    Annotation(Annotation),
    Frac(Frac),
    Ident(Ident),
    Num(Num),
    Operator(Operator),
    Padded(Padded),
    Semantics(Semantics),
    Space(Space),
    Table(Table),
    Text(Text),
    Matrix(Matrix),
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MathMl {
    content: Vec<Tag>,
    // TODO: decide what fields should go inside
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

macro_rules! tag_from_type {
    ($variant:ident => $type:path) => {
        impl From<$type> for crate::Tag {
            fn from(value: $type) -> Self {
                Self::$variant(value)
            }
        }
    };
}

pub(crate) use {from_types, tag_from_type};
