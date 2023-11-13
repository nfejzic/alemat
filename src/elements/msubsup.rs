use crate::{attributes::Attribute, MathMl};

/// The `msub`, `msup` and `msubsup` elements are used to attach subscript and superscript to a MathML
/// expression. They accept the global [`Attribute`]s.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubSup {
    base: MathMl,
    sub: Option<MathMl>,
    sup: Option<MathMl>,
    attributes: Vec<Attribute>,
}
