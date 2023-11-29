use std::marker::PhantomData;

use crate::{
    attributes::Attribute,
    markers::{Init, Uninit},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpForm {
    Infix,
    Prefix,
    Postfix,
}

impl std::fmt::Display for OpForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpForm::Infix => f.write_str("infix"),
            OpForm::Prefix => f.write_str("prefix"),
            OpForm::Postfix => f.write_str("postfix"),
        }
    }
}

/// Attribute for the `mo` (`Operator`) element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OperatorAttr {
    /// One of the global attributes.
    Global(Attribute),

    /// Either `infix`, `prefix` or `postfix`.
    Form(OpForm),

    /// A `bool` specifying whether the operator is a fence (such as parentheses). There is no
    /// visual effect for this attribute. Attribute's presence means that it's set to `true`.
    Fence,

    /// A `bool` specifying whether the operator is a separator (such as commas). There is no
    /// visual effect for this attribute. Attribute's presence means that it's set to `true`.
    Separator,

    /// A `lspace`
    /// [`<length-percentage>`](https://www.w3.org/TR/css-values-4/#typedef-length-percentage)
    /// indicating amount of space before the operator.
    LeftSpace(String),

    /// A `rspace` [`<length-percentage>`] indicating the amount of space after the operator.
    RightSpace(String),

    /// A `maxsize`
    /// [`<length-percentage>`](https://www.w3.org/TR/css-values-4/#typedef-length-percentage)
    /// indicating the maximum size of the operator when it is stretchy.
    MaxSize(String),

    /// A `minsize`
    /// [`<length-percentage>`](https://www.w3.org/TR/css-values-4/#typedef-length-percentage)
    /// indicating the minimum size of the operator when it is stretchy.
    MinSize(String),

    /// A `bool` indicating whether the operator stretches to the size of the adjacent element.
    Stretchy,

    /// A `bool` indicating whether a stretchy operator should be vertically symmetric around the
    /// imaginary math axis (centered fraction line).
    Symmetric,

    /// Either `true` or `false`. In this implementation, the attribute is `true` if present.
    LargeOp,

    /// Either `true` or `false`. In this implementation, the attribute is `true` if present.
    MovableLimits,
}

impl From<Attribute> for OperatorAttr {
    fn from(value: Attribute) -> Self {
        Self::Global(value)
    }
}

/// The `mo` element represents an operator or anything that should be rendered as an operator. In
/// general, the notational conventions for mathematical operators are quite complicated, and
/// therefore MathML provides a relatively sophisticated mechanism for specifying the rendering
/// behavior of an `<mo>` element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Operator {
    op: String,
    attributes: Vec<OperatorAttr>,
}

impl<T> From<T> for Operator
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self {
            op: value.into(),
            attributes: Default::default(),
        }
    }
}

impl Operator {
    pub fn builder() -> OperatorBuilder<Uninit> {
        OperatorBuilder::default()
    }

    pub fn op(&self) -> &str {
        &self.op
    }

    pub fn attributes(&self) -> &[OperatorAttr] {
        &self.attributes
    }
}

crate::element_from_type!(Operator => Operator);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OperatorBuilder<T> {
    op: Option<String>,
    attr: Vec<OperatorAttr>,
    _marker: PhantomData<(T,)>,
}

impl<T> OperatorBuilder<T> {
    pub fn op(self, op: impl Into<String>) -> OperatorBuilder<Init> {
        OperatorBuilder {
            op: Some(op.into()),
            attr: self.attr,
            _marker: PhantomData,
        }
    }

    pub fn attr<I, A>(mut self, attr: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<OperatorAttr>,
    {
        self.attr.extend(attr.into_iter().map(Into::into));
        self
    }
}

impl OperatorBuilder<Init> {
    pub fn build(self) -> Operator {
        Operator {
            op: self.op.expect("Op is guaranteed to be init."),
            attributes: self.attr,
        }
    }
}
