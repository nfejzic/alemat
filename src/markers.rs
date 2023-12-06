//! Marker types used in builders.

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[doc(hidden)]
/// Marker type used in builders to indicate that a particular value has been initialized.
/// This is not intended to be used dirctly.
pub struct Init;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[doc(hidden)]
/// Marker type used in builders to indicate that a particular value has not been initialized.
/// This is not intended to be used dirctly.
pub struct Uninit;
