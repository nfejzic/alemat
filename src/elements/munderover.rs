use crate::{attributes::Attribute, MathMl};

/// The `munderover` element accepts global attributes as well as `accent` and `accentunder`.
///
/// Similarly, the `mover` element (respectively `munder` element) accepts global attributes as
/// well as the `accent` attribute (respectively the `accentunder` attribute).
///
/// `accent`, `accentunder` attributes, if present, must have values that are booleans. If these
/// attributes are absent or invalid, they are treated as equal to false.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnderOverAttr {
    AccentUnder,
    AccentOver,
    Global(Attribute),
}

/// The munder, mover and munderover elements are used to attach accents or limits placed under or
/// over a MathML expression.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnderOver {
    expr: MathMl,
    over: Option<MathMl>,
    under: Option<MathMl>,
    attributes: Vec<UnderOverAttr>,
}
