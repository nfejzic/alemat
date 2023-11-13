use crate::{attributes::Attribute, MathMl};

/// The radical elements construct an expression with a root symbol `√` with a line over the content.
/// The msqrt element is used for square roots, while the mroot element is used to draw radicals
/// with indices, e.g. a cube root.
///
/// The `msqrt` and `mroot` elements accept the global [`Attribute`]s.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Radicals {
    /// The index of the radical, e.g. the 3 in `∛`.
    index: String,
    content: MathMl,
    attributes: Vec<Attribute>,
}
