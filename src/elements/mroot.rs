use std::marker::PhantomData;

use crate::{
    attributes::Attribute,
    markers::{Init, Uninit},
    Element, Elements,
};

use super::{grouping::Row, IntoElements};

/// The radical elements construct an expression with a root symbol `√` with a line over the content.
/// The msqrt element is used for square roots, while the mroot element is used to draw radicals
/// with indices, e.g. a cube root.
///
/// The `msqrt` and `mroot` elements accept the global [`Attribute`]s.
///
/// The `msqrt` and `mroot` elements sets math-shift to compact. The `mroot` element increments
/// scriptlevel by 2, and sets displaystyle to `false` in all but its first child. The user agent
/// stylesheet must contain the following rule in order to implement that behavior:
///
/// ```css
/// mroot > :not(:first-child) {
///   math-depth: add(2);
///   math-style: compact;
/// }
/// mroot, msqrt {
///   math-shift: compact;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Radical {
    /// The index of the radical, e.g. the 3 in `∛`.
    index: Elements,
    content: Elements,
    attributes: Vec<Attribute>,
}

impl Radical {
    /// Create a builder for the [`Radical`] element.
    pub fn builder() -> RadicalsBuilder<Uninit, Uninit> {
        RadicalsBuilder::default()
    }

    /// Get a reference to the index of the radical. e.g. "2" for the square root.
    pub fn index(&self) -> &Elements {
        &self.index
    }

    /// Check if the radical is a square root.
    pub fn is_square(&self) -> bool {
        if self.index().len() != 1 {
            return false;
        }

        match &self.index[0] {
            Element::Num(num) => num.num().parse::<f32>().is_ok_and(|val| val == 2f32),
            _ => false,
        }
    }

    /// Get a reference to the inner content of the [`Radical`] element.
    pub fn content(&self) -> &[Element] {
        &self.content
    }

    /// Get a reference to all attributes of the [`Radical`] element.
    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

crate::element_from_type!(Radical => Radical);

/// Builder of the [`Radical`] element.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RadicalsBuilder<T1, T2> {
    index: Option<Elements>,
    content: Option<Elements>,
    attr: Vec<Attribute>,

    _marker: PhantomData<(T1, T2)>,
}

impl<T1, T2> RadicalsBuilder<T1, T2> {
    /// Set the index of the radical. e.g. "2" for the square root.
    ///
    /// You can pass as may children as you want, but if you pass more than two with an index other
    /// than "2" they will be wrapped in a `mrow`. If you use [`alemat::row!`] to wrap children in
    /// a row yourself, they won't be wrapped again.
    pub fn index(self, index: impl IntoElements) -> RadicalsBuilder<Init, T2> {
        let mut index = index.into_elements();

        if index.len() > 1 {
            index = Row::from(index).into_elements();
        }

        RadicalsBuilder {
            index: Some(index.into_elements()),
            content: self.content,
            attr: self.attr,
            _marker: PhantomData,
        }
    }

    /// `mroot` and `msqrt` behave differently when they have multiple children:
    ///
    /// * `msqrt` can have any number of chldren
    /// * `mroot` needs to have exactly two children:
    ///   - the first is the base
    ///   - the second is the index
    ///
    /// You can pass as may children as you want, but if you pass more than two with an index other
    /// than "2" they will be wrapped in a `mrow`. If you use [`alemat::row!`] to wrap children in
    /// a row yourself, they won't be wrapped again.
    ///
    /// [`alemat::row!`]: crate::row
    pub fn content(self, content: impl IntoElements) -> RadicalsBuilder<T1, Init> {
        RadicalsBuilder {
            index: self.index,
            content: Some(content.into_elements()),
            attr: self.attr,
            _marker: PhantomData,
        }
    }

    /// Add attributes.
    pub fn attr<I, A>(mut self, attr: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        self.attr.extend(attr.into_iter().map(Into::into));
        self
    }
}

impl RadicalsBuilder<Init, Init> {
    /// Build the [`Radical`] element.
    pub fn build(self) -> Radical {
        let mut radical = Radical {
            index: self.index.expect("Index is guaranteed to be init."),
            content: self.content.expect("Content is guaranteed to be init."),
            attributes: self.attr,
        };

        if !radical.is_square() && radical.content.len() > 1 {
            let row = Row::from(radical.content);
            radical.content = Elements(vec![Element::Row(row)]);
        }

        radical
    }
}
