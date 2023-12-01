//! Elements found in the MathML Core Specification. These elements are called MathML Core
//! elements.

mod maction;
mod math;
mod merror;
mod mmultiscripts;
mod mphantom;
mod mrow;
mod mstyle;

pub mod grouping {
    pub use super::maction::*;
    pub use super::math::*;
    pub use super::merror::*;
    pub use super::mmultiscripts::Prescripts;
    pub use super::mphantom::*;
    pub use super::mrow::*;
    pub use super::mstyle::*;
}

mod mroot;

pub mod radicals {
    pub use super::mroot::*;
}

mod msubsup;
mod munderover;

pub mod scripted {
    pub use super::mmultiscripts::Multiscripts;
    pub use super::msubsup::*;
    pub use super::munderover::*;
}

mod annotation;
mod mfrac;
mod mi;
mod mn;
mod mo;
mod mpadded;
mod ms;
mod mspace;
mod mtable;
mod mtext;

use std::ops::{Deref, DerefMut};

pub use annotation::*;
pub use mfrac::*;
pub use mi::*;
pub use mn::*;
pub use mo::*;
pub use mpadded::*;
pub use ms::*;
pub use mspace::*;
pub use mtable::*;
pub use mtext::*;

use self::{
    grouping::{Action, Error, Phantom, Prescripts, Row, Style},
    mmultiscripts::Multiscripts,
    mroot::Radical,
    msubsup::SubSup,
    scripted::UnderOver,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Element {
    Action(Action),
    Annotation(Annotation),
    Error(Error),
    Frac(Frac),
    Ident(Ident),
    Multiscripts(Multiscripts),
    Prescripts(Prescripts),
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

#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elements(pub(crate) Vec<Element>);

impl Deref for Elements {
    type Target = Vec<Element>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Elements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Elements {
    pub fn into_inner(self) -> Vec<Element> {
        self.0
    }
}

/// Create a [`Elements`] from a list of [`Element`]s.
///
/// # Example:
///
/// ```rust
/// use alemat::children;
/// use alemat::elements::{Ident, Num, Element};
/// let children: [Element; 2] = children![Ident::from("x"), Num::from(42)];
/// ```
#[macro_export]
macro_rules! children {
    ($($element:expr),* $(,)?) => {
         [$($crate::elements::Element::from($element)),*]
    }
}

pub trait IntoElements {
    /// Converts the type into elements.
    fn into_elements(self) -> Elements;
}

impl<T, const N: usize> IntoElements for [T; N]
where
    T: Into<Element>,
{
    fn into_elements(self) -> Elements {
        Elements(self.into_iter().map(Into::into).collect())
    }
}

impl<T> IntoElements for Vec<T>
where
    T: Into<Element>,
{
    fn into_elements(self) -> Elements {
        Elements(self.into_iter().map(Into::into).collect())
    }
}

macro_rules! element_from_type {
    ($variant:ident => $type:path) => {
        impl From<$type> for $crate::Element {
            fn from(value: $type) -> Self {
                Self::$variant(value)
            }
        }

        impl $crate::elements::IntoElements for $type {
            fn into_elements(self) -> $crate::Elements {
                $crate::Elements(vec![$crate::Element::$variant(self)])
            }
        }
    };
}

pub(crate) use element_from_type;
