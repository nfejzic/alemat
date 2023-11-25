use std::marker::PhantomData;

use crate::{
    attributes::Attribute,
    markers::{Init, Uninit},
    MathMl,
};

/// The `msub`, `msup` and `msubsup` elements are used to attach subscript and superscript to a MathML
/// expression. They accept the global [`Attribute`]s.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubSup {
    base: MathMl,
    sub: Option<MathMl>,
    sup: Option<MathMl>,
    attributes: Vec<Attribute>,
}

crate::tag_from_type!(SubSup => SubSup);

impl SubSup {
    pub fn builder() -> SubSupBuilder<Uninit, Uninit> {
        SubSupBuilder::default()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubSupBuilder<T1, T2> {
    base: Option<MathMl>,
    sub: Option<MathMl>,
    sup: Option<MathMl>,
    attr: Vec<Attribute>,
    _marker: PhantomData<(T1, T2)>,
}

impl<T1, T2> SubSupBuilder<T1, T2> {
    pub fn base(self, base: impl Into<MathMl>) -> SubSupBuilder<Init, T2> {
        SubSupBuilder {
            base: Some(base.into()),
            sub: self.sub,
            sup: self.sup,
            attr: self.attr,
            _marker: PhantomData,
        }
    }

    pub fn subscript(self, sub: impl Into<MathMl>) -> SubSupBuilder<T1, Init> {
        SubSupBuilder {
            base: self.base,
            sub: Some(sub.into()),
            sup: self.sup,
            attr: self.attr,
            _marker: PhantomData,
        }
    }

    pub fn supscript(self, sub: impl Into<MathMl>) -> SubSupBuilder<T1, Init> {
        SubSupBuilder {
            base: self.base,
            sub: self.sup,
            sup: Some(sub.into()),
            attr: self.attr,
            _marker: PhantomData,
        }
    }

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
    pub fn build(self) -> SubSup {
        debug_assert!(
            self.sub.is_some() || self.sup.is_some(),
            "SubSup element must have at least one of sub or sup."
        );

        SubSup {
            base: self.base.expect("Base is guaranteed to be init."),
            sub: self.sub,
            sup: self.sup,
            attributes: self.attr,
        }
    }
}
