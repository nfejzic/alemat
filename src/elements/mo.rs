use crate::attributes::Attribute;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpForm {
    Infix,
    Prefix,
    Postfix,
}

/// Attribute for the `mo` (`Operator`) element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OperatorAttr {
    /// One of the global attributes.
    Global(Attribute),

    /// Either `infix`, `prefix` or `postfix`.
    Form(OpForm),

    /// The specification does not define any observable behavior that is specific to the fence
    /// attribute.
    Fence,

    /// The specification does not define any observable behavior that is specific to the separator
    /// attribute.
    Separator,

    /// Must be a
    /// [<length-percentage>](https://www.w3.org/TR/css-values-4/#typedef-length-percentage).
    LeftSpace(String),

    /// Must be a
    /// [<length-percentage>](https://www.w3.org/TR/css-values-4/#typedef-length-percentage).
    RightSpace(String),

    /// Must be a
    /// [<length-percentage>](https://www.w3.org/TR/css-values-4/#typedef-length-percentage).
    MaxSize(String),

    /// Must be a
    /// [<length-percentage>](https://www.w3.org/TR/css-values-4/#typedef-length-percentage).
    MinSize(String),

    /// Either `true` or `false`. In this implementation, the attribute is `true` if present.
    Stretchy,

    /// Either `true` or `false`. In this implementation, the attribute is `true` if present.
    Symmetric,

    /// Either `true` or `false`. In this implementation, the attribute is `true` if present.
    LargeOp,

    /// Either `true` or `false`. In this implementation, the attribute is `true` if present.
    MovableLimits,
}

/// The mo element represents an operator or anything that should be rendered as an operator. In
/// general, the notational conventions for mathematical operators are quite complicated, and
/// therefore MathML provides a relatively sophisticated mechanism for specifying the rendering
/// behavior of an <mo> element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Operator {
    num: String,
    attributes: Vec<OperatorAttr>,
}

impl<T> From<T> for Operator
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self {
            num: value.into(),
            attributes: Default::default(),
        }
    }
}
