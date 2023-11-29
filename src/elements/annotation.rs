use std::marker::PhantomData;

use crate::{
    attributes::Attribute,
    markers::{Init, Uninit},
    Element, Elements,
};

use super::IntoElements;

/// The content of `annotation` element, either text or MathML.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnnotationContent {
    Text(String),
    Nested(Elements),
}

impl From<String> for AnnotationContent {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl<T> From<T> for AnnotationContent
where
    T: IntoElements,
{
    fn from(value: T) -> Self {
        Self::Nested(value.into_elements())
    }
}

impl From<Element> for AnnotationContent {
    fn from(value: Element) -> Self {
        Self::Nested(Elements(vec![value]))
    }
}

/// An attribute of `annotation` element. Either one of the global [`Attribute`]s, or `encode`
/// attribute.
///
/// [`Attribute`]: crate::attributes::Attribute
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnnotationAttr {
    Global(Attribute),

    /// NOTE: Authors can use the encoding attribute to distinguish annotations for HTML
    /// integration point, clipboard copy, alternative rendering, etc. In particular, CSS can be
    /// used to render alternative annotations e.g.
    ///
    /// ```css
    /// /* Hide the annotated child. */
    /// semantics > :first-child { display: none; }
    ///  /* Show all text annotations. */
    /// semantics > annotation { display: inline; }
    /// /* Show all HTML annotations. */
    /// semantics > annotation-xml[encoding="text/html" i],
    /// semantics > annotation-xml[encoding="application/xhtml+xml" i] {
    /// display: inline-block;
    /// }
    /// ```
    Encoding(String),
}

impl From<Attribute> for AnnotationAttr {
    fn from(value: Attribute) -> Self {
        Self::Global(value)
    }
}

/// The `annotation` (and `annotation-xml`) element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Annotation {
    content: AnnotationContent,
    attributes: Vec<AnnotationAttr>,
}

crate::element_from_type!(Annotation => Annotation);

impl Annotation {
    pub fn builder() -> AnnotationBuilder<Uninit> {
        AnnotationBuilder::default()
    }

    pub fn content(&self) -> &AnnotationContent {
        &self.content
    }

    pub fn attributes(&self) -> &[AnnotationAttr] {
        &self.attributes
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

    pub fn attr<I, A>(mut self, attr: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<AnnotationAttr>,
    {
        self.attr.extend(attr.into_iter().map(Into::into));
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
    children: Elements,

    /// The `semantics` element accepts the global [`Attribute`]s.
    attr: Vec<Attribute>,
}

impl Semantics {
    pub fn builder() -> SemanticsBuilder<Uninit> {
        SemanticsBuilder::default()
    }

    pub fn children(&self) -> &[Element] {
        &self.children
    }

    pub fn attributes(&self) -> &[Attribute] {
        &self.attr
    }
}

crate::element_from_type!(Semantics => Semantics);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticsBuilder<T> {
    content: Option<Elements>,
    attr: Vec<Attribute>,

    _marker: PhantomData<(T,)>,
}

impl<T> SemanticsBuilder<T> {
    pub fn content(self, content: impl IntoElements) -> SemanticsBuilder<Init> {
        SemanticsBuilder {
            content: Some(content.into_elements()),
            attr: self.attr,
            _marker: PhantomData,
        }
    }

    pub fn attr<A>(mut self, attr: A) -> Self
    where
        A: IntoIterator<Item = Attribute>,
    {
        self.attr.extend(attr);
        self
    }
}

impl SemanticsBuilder<Init> {
    pub fn build(self) -> Semantics {
        Semantics {
            children: self
                .content
                .expect("Content is guaranteed to be initialized at compile time."),
            attr: self.attr,
        }
    }
}
