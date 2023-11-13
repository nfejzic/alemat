use crate::attributes::Attribute;

/// The `mspace` element accepts the global [`Attribute`]s as well as `width`, `height` and `depth`.
///
/// The `width`, `height`, `depth`, if present, must have a value that is a valid
/// [length-percentage](https://www.w3.org/TR/css-values-4/#typedef-length-percentage).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpaceAttr {
    /// If present, valid and not a percentage then it is used as a presentational hint setting the
    /// element's width property to the corresponding value.
    Width(String),

    /// If absent, invalid or a percentage then the requested line-ascent is 0. Otherwise the
    /// requested line-ascent is the resolved value of the height attribute, clamping negative
    /// values to 0.
    Height(String),

    /// If both the height and depth attributes are present, valid and not a percentage then they
    /// are used as a presentational hint setting the element's height property to the
    /// concatenation of the strings "calc(", the height attribute value, " + ", the depth
    /// attribute value, and ")".
    ///
    /// If only one of these attributes is present, valid and not a
    /// percentage then it is treated as a presentational hint setting the element's height
    /// property to the corresponding value.
    Depth(String),

    /// One of the global [`Attribute`]s.
    Global(Attribute),
}

/// The mspace empty element represents a blank space of any desired size, as set by its
/// attributes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Space {
    attr: Vec<SpaceAttr>,
}
