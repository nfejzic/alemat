use std::marker::PhantomData;

use crate::attributes::Attribute;
use crate::markers::{Init, Uninit};
use crate::{Element, Elements};

use super::grouping::Row;
use super::IntoElements;

/// An attribute of `mfrac` element. Either one of the global [`Attribute`]s, or `linethickness`
/// attribute.
///
/// [`Attribute`]: crate::attributes::Attribute
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FracAttr {
    /// Global attribute.
    Global(Attribute),

    /// The `linethickness` attribute indicates the fraction line thickness to use for the fraction
    /// bar.
    /// It must have a value that is a valid
    /// [`<length-percentage>`](https://www.w3.org/TR/css-values-4/#typedef-length-percentage).
    LineThickness(String),
}

/// The merror element displays its contents as an ”error message”. The intent of this element is
/// to provide a standard way for programs that generate MathML from other input to report syntax
/// errors in their input.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frac {
    // regular comment
    num: Elements,
    denom: Elements,
    attributes: Vec<FracAttr>,
}

impl Frac {
    /// Create the builder for [`Frac`] element.
    pub fn builder() -> FracBuilder<Uninit, Uninit> {
        FracBuilder::default()
    }

    /// Get a reference to the content of the numerator.
    pub fn num(&self) -> &[Element] {
        &self.num
    }

    /// Get a reference to the content of the denominator.
    pub fn denom(&self) -> &[Element] {
        &self.denom
    }

    /// Get a reference to all attributes of the [`Frac`] element.
    pub fn attributes(&self) -> &[FracAttr] {
        &self.attributes
    }
}

impl<N, D> From<(N, D)> for Frac
where
    N: IntoElements,
    D: IntoElements,
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
            num: num.into_elements(),
            denom: denom.into_elements(),
            attributes: Default::default(),
        }
    }
}

crate::element_from_type!(Frac => Frac);

/// Builder for [`Frac`]. It uses static type checking to ensure that all required fields have been
/// initialized. Only then is the `build` function available.
///
/// # Example
///
/// ```rust
/// use alemat::elements::Frac;
/// use alemat::elements::Num;
///
/// let frac = Frac::builder().num(Num::from(1)).denom(Num::from(2)).build();
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FracBuilder<N, D> {
    num: Option<Elements>,
    denom: Option<Elements>,
    attr: Vec<FracAttr>,

    _marker: PhantomData<(N, D)>,
}

impl<N, D> FracBuilder<N, D> {
    /// Add or overwrite the numerator to the `mfrac` element.
    pub fn num(self, num: impl IntoElements) -> FracBuilder<Init, D> {
        let mut num = num.into_elements();

        if num.len() > 1 {
            num = Row::from(num).into_elements();
        }

        FracBuilder {
            num: Some(num.into_elements()),
            denom: self.denom,
            attr: self.attr,
            _marker: PhantomData,
        }
    }

    /// Add or overwrite the denominator to the `mfrac` element.
    pub fn denom(self, denom: impl IntoElements) -> FracBuilder<N, Init> {
        let mut denom = denom.into_elements();

        if denom.len() > 1 {
            denom = Row::from(denom).into_elements();
        }

        FracBuilder {
            num: self.num,
            denom: Some(denom.into_elements()),
            attr: self.attr,
            _marker: PhantomData,
        }
    }

    /// Add attributes to the `mfrac` element. Previous attributes will not be overwritten.
    pub fn attr<A>(mut self, attr: A) -> Self
    where
        A: IntoIterator<Item = FracAttr>,
    {
        self.attr.extend(attr);
        self
    }
}

impl FracBuilder<Init, Init> {
    /// Build the [`Frac`] element.
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
