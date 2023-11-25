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
