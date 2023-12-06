use std::marker::PhantomData;

use crate::{
    attributes::Attribute,
    markers::{Init, Uninit},
    Element, Elements,
};

use super::IntoElements;

/// The `maction` element accepts the global [`Attribute`]s as well as `selection` and
/// `actiontype`.
///
/// [`Attribute`]: crate::attributes::Attribute
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ActionAttr {
    /// The of the global [`Attribute`]s.
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
    content: Elements,
    attributes: Vec<ActionAttr>,
}

impl From<Elements> for Action {
    fn from(value: Elements) -> Self {
        Self {
            content: value,
            attributes: Default::default(),
        }
    }
}

impl Action {
    /// Create new `maction` element.
    pub fn with_mathml(math: impl IntoElements) -> Self {
        Self {
            content: math.into_elements(),
            attributes: Default::default(),
        }
    }

    /// Create a builder for [`Action`] element.
    pub fn builder() -> ActionBuilder<Uninit> {
        ActionBuilder::default()
    }

    /// Get a reference to the inner content of the [`Action`] element.
    pub fn content(&self) -> &[Element] {
        &self.content
    }

    /// Get a reference to all attributes of the [`Action`] element.
    pub fn attributes(&self) -> &[ActionAttr] {
        &self.attributes
    }
}

crate::element_from_type!(Action => Action);

/// Builder of the [`Action`] element.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ActionBuilder<T> {
    content: Option<Elements>,
    attributes: Vec<ActionAttr>,

    _marker: PhantomData<(T,)>,
}

impl<T> ActionBuilder<T> {
    /// Set the content of the [`Action`] element.
    pub fn content(self, content: impl IntoElements) -> ActionBuilder<Init> {
        ActionBuilder {
            content: Some(content.into_elements()),
            attributes: self.attributes,
            _marker: PhantomData,
        }
    }

    /// Add attributes.
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
    /// Build the [`Action`] element.
    pub fn build(self) -> Action {
        Action {
            content: self.content.expect("Content is guaranteed to be init."),
            attributes: self.attributes,
        }
    }
}
