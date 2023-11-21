use std::marker::PhantomData;

use crate::{
    attributes::Attribute,
    markers::{Init, Uninit},
    MathMl,
};

/// The `maction` element accepts the global [`Attribute`]s as well as `selection` and
/// `actiontype`.
///
/// [`Attribute`]: crate::attributes::Attribute
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ActionAttr {
    Global(Attribute),
    /// The child element currently visible, only taken into account for `actiontype="toggle"` or
    /// non-standard actiontype values. The default value is `1`, which is the first child element.
    Selection(String),

    /// The action which specifies what happens for this element. Special behavior for the
    /// following values were implemented by some browsers:
    ///
    /// * `statusline`: If there is a click on the expression or the reader moves the pointer over
    ///   it, the message is sent to the browser's status line. The syntax is:
    ///
    ///   ```html
    ///   <maction actiontype="statusline"> expression message </maction>.
    ///   ```
    ///
    /// * `toggle`: When there is a click on the subexpression, the rendering alternates the
    ///    display of selected subexpressions. Therefore each click increments the selection value.
    ///    The syntax is:
    ///
    ///   ```html
    ///   <maction actiontype="toggle" selection="positive-integer">
    ///       expression1 expression2 expressionN
    ///   </maction>.
    ///   ```
    ActionType(String),
}

/// Historically, the `maction` element provides a mechanism for binding actions to expressions.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Action {
    content: MathMl,
    attributes: Vec<ActionAttr>,
}

impl Action {
    /// Create new `maction` element.
    pub fn with_mathml(math: impl Into<MathMl>) -> Self {
        Self {
            content: math.into(),
            attributes: Default::default(),
        }
    }

    pub fn builder() -> ActionBuilder<Uninit> {
        ActionBuilder::default()
    }
}

impl<T> From<T> for Action
where
    T: Into<MathMl>,
{
    fn from(value: T) -> Self {
        Self {
            content: value.into(),
            attributes: Default::default(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ActionBuilder<T> {
    content: Option<MathMl>,
    attributes: Vec<ActionAttr>,

    _marker: PhantomData<(T,)>,
}

impl<T> ActionBuilder<T> {
    pub fn content(self, content: impl Into<MathMl>) -> ActionBuilder<Init> {
        ActionBuilder {
            content: Some(content.into()),
            attributes: self.attributes,
            _marker: PhantomData,
        }
    }

    pub fn attr<I, A>(mut self, attributes: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<ActionAttr>,
    {
        self.attributes
            .extend(attributes.into_iter().map(Into::into));

        self
    }
}

impl ActionBuilder<Init> {
    pub fn build(self) -> Action {
        Action {
            content: self.content.expect("Content is guaranteed to be init."),
            attributes: self.attributes,
        }
    }
}
