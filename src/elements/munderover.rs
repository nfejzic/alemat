use std::marker::PhantomData;

use crate::{
    attributes::Attribute,
    markers::{Init, Uninit},
    MathMl,
};

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

/// The munder, mover and munderover elements are used to attach accents or limits placed under or
/// over a MathML expression.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnderOver {
    expr: MathMl,
    over: Option<MathMl>,
    under: Option<MathMl>,
    attributes: Vec<UnderOverAttr>,
}

impl UnderOver {
    pub fn builder() -> UnderOverBuilder<Uninit, Uninit> {
        UnderOverBuilder::default()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnderOverBuilder<T1, T2> {
    expr: Option<MathMl>,
    under: Option<MathMl>,
    over: Option<MathMl>,
    attr: Vec<UnderOverAttr>,

    _marker: PhantomData<(T1, T2)>,
}

impl<T1, T2> UnderOverBuilder<T1, T2> {
    pub fn expr(self, expr: impl Into<MathMl>) -> UnderOverBuilder<Init, T2> {
        UnderOverBuilder {
            expr: Some(expr.into()),
            under: self.under,
            over: self.over,
            attr: self.attr,

            _marker: PhantomData,
        }
    }

    pub fn over(self, over: impl Into<MathMl>) -> UnderOverBuilder<T1, Init> {
        UnderOverBuilder {
            expr: self.expr,
            under: self.under,
            over: Some(over.into()),
            attr: self.attr,
            _marker: PhantomData,
        }
    }

    pub fn under(self, under: impl Into<MathMl>) -> UnderOverBuilder<T1, Init> {
        UnderOverBuilder {
            expr: self.expr,
            under: Some(under.into()),
            over: self.over,
            attr: self.attr,
            _marker: PhantomData,
        }
    }

    pub fn attr<I, A>(mut self, attr: I) -> UnderOverBuilder<T1, T2>
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

        UnderOver {
            expr: self.expr.expect("Expr is guaranteed to be init."),
            over: self.over,
            under: self.under,
            attributes: self.attr,
        }
    }
}
