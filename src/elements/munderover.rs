use std::marker::PhantomData;

use crate::{
    attributes::Attribute,
    elements::grouping::Row,
    markers::{Init, Uninit},
    Element, Elements,
};

use super::IntoElements;

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum UnderOverInner {
    Under(Elements),
    Over(Elements),
    UnderOver { under: Elements, over: Elements },
}

/// The munder, mover and munderover elements are used to attach accents or limits placed under or
/// over a MathML expression.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnderOver {
    expr: Elements,
    inner: UnderOverInner,
    attributes: Vec<UnderOverAttr>,
}

impl UnderOver {
    pub fn builder() -> UnderOverBuilder<Uninit, Uninit> {
        UnderOverBuilder::default()
    }

    pub fn expr(&self) -> &[Element] {
        &self.expr
    }

    pub fn under(&self) -> Option<&[Element]> {
        match self.inner {
            UnderOverInner::Under(ref under) | UnderOverInner::UnderOver { ref under, .. } => {
                Some(under)
            }
            _ => None,
        }
    }

    pub fn over(&self) -> Option<&[Element]> {
        match self.inner {
            UnderOverInner::Over(ref over) | UnderOverInner::UnderOver { ref over, .. } => {
                Some(over)
            }
            _ => None,
        }
    }

    pub fn attributes(&self) -> &[UnderOverAttr] {
        &self.attributes
    }
}

crate::element_from_type!(UnderOver => UnderOver);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnderOverBuilder<T1, T2> {
    expr: Option<Elements>,
    under: Option<Elements>,
    over: Option<Elements>,
    attr: Vec<UnderOverAttr>,

    _marker: PhantomData<(T1, T2)>,
}

impl<T1, T2> UnderOverBuilder<T1, T2> {
    pub fn expr(self, expr: impl IntoElements) -> UnderOverBuilder<Init, T2> {
        UnderOverBuilder {
            expr: Some(expr.into_elements()),
            under: self.under,
            over: self.over,
            attr: self.attr,

            _marker: PhantomData,
        }
    }

    pub fn over(self, over: impl IntoElements) -> UnderOverBuilder<T1, Init> {
        UnderOverBuilder {
            expr: self.expr,
            under: self.under,
            over: Some(over.into_elements()),
            attr: self.attr,
            _marker: PhantomData,
        }
    }

    pub fn under(self, under: impl IntoElements) -> UnderOverBuilder<T1, Init> {
        UnderOverBuilder {
            expr: self.expr,
            under: Some(under.into_elements()),
            over: self.over,
            attr: self.attr,
            _marker: PhantomData,
        }
    }

    pub fn attr<I, A>(mut self, attr: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<UnderOverAttr>,
    {
        self.attr.extend(attr.into_iter().map(Into::into));
        self
    }
}

impl UnderOverBuilder<Init, Init> {
    pub fn build(self) -> UnderOver {
        debug_assert!(
            self.over.is_some() || self.under.is_some(),
            "At least one of 'over' or 'under' must be initialized."
        );

        let inner = match (self.under, self.over) {
            (None, Some(over)) => UnderOverInner::Over(over),
            (Some(under), None) => UnderOverInner::Under(under),
            (Some(under), Some(over)) => UnderOverInner::UnderOver { under, over },

            (None, None) => {
                unreachable!("T2 set to Init guarantees that at least sub or sup are initialized.")
            }
        };

        let mut expr = self.expr.expect("Expr is guaranteed to be init.");

        if expr.len() > 1 {
            expr = Row::from(expr).into_elements();
        }

        UnderOver {
            expr,
            inner,
            attributes: self.attr,
        }
    }
}
