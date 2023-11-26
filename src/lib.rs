//! Library for type-safe building of MathML.

pub mod attributes;
pub mod elements;
pub mod markers;
mod to_mathml;

pub use to_mathml::*;

use elements::{
    grouping::{Action, Error, Phantom, Row, Style},
    radicals::Radical,
    scripted::{Multiscripts, SubSup, UnderOver},
    Annotation, Frac, Ident, Num, Operator, Padded, Semantics, Space, StrLiteral, Table, Text,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Tag {
    Action(Action),
    Annotation(Annotation),
    Error(Error),
    Frac(Frac),
    Ident(Ident),
    Multiscripts(Multiscripts),
    Num(Num),
    Operator(Operator),
    Padded(Padded),
    Phantom(Phantom),
    Radical(Radical),
    Row(Row),
    Semantics(Semantics),
    Space(Space),
    StrLiteral(StrLiteral),
    Style(Style),
    SubSup(SubSup),
    Table(Table),
    Text(Text),
    UnderOver(UnderOver),
}

impl ToMathMl for Tag {
    fn to_mathml(&self) -> String {
        match self {
            Self::Action(el) => unimplemented!("ToMathMl not implemented for {el:?}"),
            Self::Annotation(el) => unimplemented!("ToMathMl not implemented for {el:?}"),
            Self::Error(el) => unimplemented!("ToMathMl not implemented for {el:?}"),
            Self::Frac(el) => unimplemented!("ToMathMl not implemented for {el:?}"),
            Self::Ident(el) => unimplemented!("ToMathMl not implemented for {el:?}"),
            Self::Multiscripts(el) => unimplemented!("ToMathMl not implemented for {el:?}"),
            Self::Num(el) => unimplemented!("ToMathMl not implemented for {el:?}"),
            Self::Operator(el) => unimplemented!("ToMathMl not implemented for {el:?}"),
            Self::Padded(el) => unimplemented!("ToMathMl not implemented for {el:?}"),
            Self::Phantom(el) => unimplemented!("ToMathMl not implemented for {el:?}"),
            Self::Radical(el) => unimplemented!("ToMathMl not implemented for {el:?}"),
            Self::Row(el) => unimplemented!("ToMathMl not implemented for {el:?}"),
            Self::Semantics(el) => unimplemented!("ToMathMl not implemented for {el:?}"),
            Self::Space(el) => unimplemented!("ToMathMl not implemented for {el:?}"),
            Self::StrLiteral(el) => unimplemented!("ToMathMl not implemented for {el:?}"),
            Self::Style(el) => unimplemented!("ToMathMl not implemented for {el:?}"),
            Self::SubSup(el) => unimplemented!("ToMathMl not implemented for {el:?}"),
            Self::Table(el) => unimplemented!("ToMathMl not implemented for {el:?}"),
            Self::Text(el) => unimplemented!("ToMathMl not implemented for {el:?}"),
            Self::UnderOver(el) => unimplemented!("ToMathMl not implemented for {el:?}"),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MathMl {
    content: Vec<Tag>,
    // TODO: decide what fields should go inside
}

impl ToMathMl for MathMl {
    fn to_mathml(&self) -> String {
        let content = self
            .content
            .iter()
            .map(ToMathMl::to_mathml)
            .collect::<String>();

        format!("<math>{content}</math>")
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
