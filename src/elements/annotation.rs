use std::marker::PhantomData;

use crate::{
    attributes::Attribute,
    markers::{Init, Uninit},
    MathMl,
};

/// The content of `annotation` element, either text or MathML.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnnotationContent {
    Text(String),
    MathMl(MathMl),
}

impl From<String> for AnnotationContent {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl<T> From<T> for AnnotationContent
where
    T: Into<MathMl>,
{
    fn from(value: T) -> Self {
        Self::MathMl(value.into())
    }
}

/// An attribute of `annotation` element. Either one of the global [`Attribute`]s, or `encode`
/// attribute.
///
/// [`Attribute`]: crate::attributes::Attribute
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnnotationAttr {
    Global(Attribute),
    Encoding(String),
}

/// The `annotation` (and `annotation-xml`) element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Annotation {
    content: AnnotationContent,
    attributes: Vec<AnnotationAttr>,
}

impl Annotation {
    pub fn builder() -> AnnotationBuilder<Uninit> {
        AnnotationBuilder {
            content: None,
            ..Default::default()
        }
    }
}

impl<T> From<T> for Annotation
where
    T: Into<AnnotationContent>,
{
    fn from(value: T) -> Self {
        Self {
            content: value.into(),
            attributes: Default::default(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AnnotationBuilder<T> {
    content: Option<AnnotationContent>,
    attr: Vec<AnnotationAttr>,

    _marker: PhantomData<(T,)>,
}

impl<T> AnnotationBuilder<T> {
    pub fn content(self, content: impl Into<AnnotationContent>) -> AnnotationBuilder<Init> {
        AnnotationBuilder {
            content: Some(content.into()),
            attr: self.attr,
            _marker: PhantomData,
        }
    }

    pub fn attr<A>(mut self, attr: A) -> AnnotationBuilder<T>
    where
        A: IntoIterator<Item = AnnotationAttr>,
    {
        self.attr.extend(attr);
        self
    }
}

impl AnnotationBuilder<Init> {
    pub fn build(self) -> Annotation {
        Annotation {
            content: self
                .content
                .expect("Content is guaranteed to be initialized at compile time."),
            attributes: self.attr,
        }
    }
}

/// The `semantics` element is the container element that associates annotations with a MathML
/// expression. Typically, the `semantics` element has as its first child element a MathML
/// expression to be annotated while subsequent child elements represent text annotations within an
/// `annotation` element, or more complex markup annotations within an `annotation-xml` element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Semantics {
    /// Children of the `semantics` element. Rendering is same as `mrow`.
    children: MathMl,

    /// The `semantics` element accepts the global [`Attribute`]s.
    attr: Vec<Attribute>,
}
