use std::marker::PhantomData;

use crate::{
    attributes::Attribute,
    markers::{Init, Uninit},
    Element, Elements,
};

use super::{grouping::Row, IntoElements};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum SubSupInner {
    Sub(Elements),
    Sup(Elements),
    SubSup { sub: Elements, sup: Elements },
}

/// The `msub`, `msup` and `msubsup` elements are used to attach subscript and superscript to a MathML
/// expression. They accept the global [`Attribute`]s.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubSup {
    base: Elements,
    inner: SubSupInner,
    attributes: Vec<Attribute>,
}

crate::element_from_type!(SubSup => SubSup);

impl SubSup {
    /// Create a builder for [`SubSup`] element.
    pub fn builder() -> SubSupBuilder<Uninit, Uninit> {
        SubSupBuilder::default()
    }

    /// Get a reference to the subscript content of the [`SubSup`] element if present.
    pub fn sub(&self) -> Option<&[Element]> {
        match self.inner {
            SubSupInner::Sub(ref sub) | SubSupInner::SubSup { ref sub, .. } => Some(sub),
            _ => None,
        }
    }

    /// Get a reference to the superscript content of the [`SubSup`] element if present.
    pub fn sup(&self) -> Option<&[Element]> {
        match self.inner {
            SubSupInner::Sup(ref sup) | SubSupInner::SubSup { ref sup, .. } => Some(sup),
            _ => None,
        }
    }

    /// Get a reference to the base content of the [`SubSup`] element.
    pub fn base(&self) -> &[Element] {
        &self.base
    }

    /// Get a reference to all attributes of the [`SubSup`] element.
    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

/// Builder of the [`SubSup`] element.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubSupBuilder<T1, T2> {
    base: Option<Elements>,
    sub: Option<Elements>,
    sup: Option<Elements>,
    attr: Vec<Attribute>,
    _marker: PhantomData<(T1, T2)>,
}

impl<T1, T2> SubSupBuilder<T1, T2> {
    /// Set the base of the [`SubSup`] element.
    pub fn base(self, base: impl IntoElements) -> SubSupBuilder<Init, T2> {
        let mut base = base.into_elements();

        if base.len() > 1 {
            base = Row::from(base).into_elements();
        }

        SubSupBuilder {
            base: Some(base.into_elements()),
            sub: self.sub,
            sup: self.sup,
            attr: self.attr,
            _marker: PhantomData,
        }
    }

    /// Set the subscript of the [`SubSup`] element.
    pub fn subscript(self, sub: impl IntoElements) -> SubSupBuilder<T1, Init> {
        let mut sub = sub.into_elements();

        if sub.len() > 1 {
            sub = Row::from(sub).into_elements();
        }

        SubSupBuilder {
            base: self.base,
            sub: Some(sub.into_elements()),
            sup: self.sup,
            attr: self.attr,
            _marker: PhantomData,
        }
    }

    /// Set the superscript of the [`SubSup`] element.
    pub fn supscript(self, sup: impl IntoElements) -> SubSupBuilder<T1, Init> {
        let mut sup = sup.into_elements();

        if sup.len() > 1 {
            sup = Row::from(sup).into_elements();
        }

        SubSupBuilder {
            base: self.base,
            sub: self.sub,
            sup: Some(sup.into_elements()),
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

impl SubSupBuilder<Init, Init> {
    /// Build the [`SubSup`] element.
    pub fn build(self) -> SubSup {
        debug_assert!(
            self.sub.is_some() || self.sup.is_some(),
            "SubSup element must have at least one of sub or sup."
        );

        let inner = match (self.sub, self.sup) {
            (None, Some(sup)) => SubSupInner::Sup(sup),
            (Some(sub), None) => SubSupInner::Sub(sub),
            (Some(sub), Some(sup)) => SubSupInner::SubSup { sub, sup },

            (None, None) => {
                unreachable!("T2 set to Init guarantees that at least sub or sup are initialized.")
            }
        };

        let base = self.base.expect("Base is guaranteed to be init.");

        SubSup {
            base,
            inner,
            attributes: self.attr,
        }
    }
}
