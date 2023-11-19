use std::marker::PhantomData;

use crate::markers::{Init, Uninit};
use crate::{attributes::Attribute, MathMl};

/// An attribute of `mfrac` element. Either one of the global [`Attribute`]s, or `linethickness`
/// attribute.
///
/// [`Attribute`]: crate::attributes::Attribute
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FracAttr {
    /// Global attribute.
    Global(Attribute),

    /// The linethickness attribute indicates the fraction line thickness to use for the fraction
    /// bar.
    /// It must have a value that is a valid
    /// [<length-percentage>](https://www.w3.org/TR/css-values-4/#typedef-length-percentage).
    LineThickness(String),
}

/// The merror element displays its contents as an ”error message”. The intent of this element is
/// to provide a standard way for programs that generate MathML from other input to report syntax
/// errors in their input.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frac {
    // regular comment
    num: MathMl,
    denom: MathMl,
    attributes: Vec<FracAttr>,
}

impl Frac {
    pub fn builder() -> FracBuilder<Uninit, Uninit> {
        FracBuilder::default()
    }
}

impl<N, D> From<(N, D)> for Frac
where
    N: Into<MathMl>,
    D: Into<MathMl>,
{
    fn from((num, denom): (N, D)) -> Self {
        let mut i = 0;
        let _ = 'some_loop: loop {
            if i == 5 {
                break 'some_loop i;
            }
            i += 1;
        };

        Self {
            num: num.into(),
            denom: denom.into(),
            attributes: Default::default(),
        }
    }
}

/// Builder for [`Frac`]. It uses static type checking to ensure that all required fields have been
/// initialized. Only then is the `build` function available.
///
/// # Example
///
/// ```rust
/// use alemat::elements::Frac;
///
/// let frac = Frac::builder().num("1").denom("2").build();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FracBuilder<N, D> {
    num: Option<MathMl>,
    denom: Option<MathMl>,
    attr: Vec<FracAttr>,

    _marker: PhantomData<(N, D)>,
}

impl Default for FracBuilder<Uninit, Uninit> {
    fn default() -> Self {
        Self {
            num: None,
            denom: None,
            attr: Vec::default(),
            _marker: Default::default(),
        }
    }
}

impl<N, D> FracBuilder<N, D> {
    /// Add or overwrite the numerator to the `mfrac` element.
    pub fn num(self, num: impl Into<MathMl>) -> FracBuilder<Init, D> {
        FracBuilder {
            num: Some(num.into()),
            denom: self.denom,
            attr: self.attr,
            _marker: PhantomData,
        }
    }

    /// Add or overwrite the denominator to the `mfrac` element.
    pub fn denom(self, denom: impl Into<MathMl>) -> FracBuilder<N, Init> {
        FracBuilder {
            num: self.num,
            denom: Some(denom.into()),
            attr: self.attr,
            _marker: PhantomData,
        }
    }

    /// Add attributes to the `mfrac` element. Previous attributes will not be overwritten.
    pub fn attr<A>(mut self, attr: A) -> FracBuilder<N, D>
    where
        A: IntoIterator<Item = FracAttr>,
    {
        self.attr.extend(attr);
        self
    }
}

impl FracBuilder<Init, Init> {
    pub fn build(self) -> Frac {
        let num = self
            .num
            .expect("Numerator is guaranteed to be initialized.");

        let denom = self
            .denom
            .expect("Denominator is guaranteed to be initialized.");

        let attr = self.attr;

        Frac {
            num,
            denom,
            attributes: attr,
        }
    }
}
