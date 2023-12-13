//! Elements found in the MathML Core Specification. These elements are called MathML Core
//! elements.

mod maction;
mod merror;
mod mmultiscripts;
mod mphantom;
mod mrow;
mod mstyle;

/// The grouping elements are `maction`, `math`, `merror`, `mphantom`, `mprescripts`, `mrow`,
/// `mstyle`, `semantics` and unknown MathML elements.
pub mod grouping {
    pub use super::maction::*;
    pub use super::merror::*;
    pub use super::mmultiscripts::Prescripts;
    pub use super::mphantom::*;
    pub use super::mrow::*;
    pub use super::mstyle::*;
}

mod mroot;

/// The radical elements are `mroot` and `msqrt`. In this implementation both are constructed using
/// the [`Radical`] struct.
pub mod radicals {
    pub use super::mroot::*;
}

mod msubsup;
mod munderover;

/// The scripted elements are `mmultiscripts`, `mover`, `msub`, `msubsup`, `msup`, `munder` and
/// `munderover`.
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

/// The MathML elements.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Element {
    /// `maction` element.
    Action(Action),

    /// `annotation` and `annotation-xml` elements.
    Annotation(Annotation),

    /// `merror` element.
    Error(Error),

    /// `mfrac` element.
    Frac(Frac),

    /// `mi` element.
    Ident(Ident),

    /// `mmultiscripts` element.
    Multiscripts(Multiscripts),

    /// `mprescripts` element.
    Prescripts(Prescripts),

    /// `mn` element.
    Num(Num),

    /// `mo` element.
    Operator(Operator),

    /// `mpadded` element.
    Padded(Padded),

    /// `mphantom` element.
    Phantom(Phantom),

    /// `mroot` and `msqrt` elements.
    Radical(Radical),

    /// `mrow` element.
    Row(Row),

    /// `msemantics` element.
    Semantics(Semantics),

    /// `mspace` element.
    Space(Space),

    /// `ms` element.
    StrLiteral(StrLiteral),

    /// `mstyle`
    Style(Style),

    /// `msub`, `msup` and `msubsup` elements.
    SubSup(SubSup),

    /// `mtable` element.
    Table(Table),

    /// `mtext` element.
    Text(Text),

    /// `munder`, `mover` and `munderover` elements.
    UnderOver(UnderOver),
}

/// A list of [`Element`]s.
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

impl FromIterator<Element> for Elements {
    fn from_iter<T: IntoIterator<Item = Element>>(iter: T) -> Self {
        let inner = iter.into_iter().collect();
        Self(inner)
    }
}

impl FromIterator<Elements> for Elements {
    fn from_iter<T: IntoIterator<Item = Elements>>(iter: T) -> Self {
        let mut elements = Self::default();
        for mut el in iter.into_iter() {
            elements.append(&mut el);
        }

        elements
    }
}

impl IntoElements for Elements {
    fn into_elements(self) -> Elements {
        self
    }
}

impl Elements {
    /// Consumes the [`Elements`] and returns a [`Vec`] of [`Element`]s.
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

/// Trait for conversion into [`Elements`].
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
