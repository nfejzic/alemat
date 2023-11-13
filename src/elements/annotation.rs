use crate::{attributes::Attribute, MathMl};

/// The content of `annotation` element, either text or MathML.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnnotationContent {
    Text(String),
    MathMl(MathMl),
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
    /// Create new `annotation` element.
    pub fn with_text(text: impl Into<String>) -> Self {
        Self {
            content: AnnotationContent::Text(text.into()),
            attributes: Default::default(),
        }
    }

    /// Create new `annotation-xml` element.
    pub fn with_mathml(math: impl Into<MathMl>) -> Self {
        Self {
            content: AnnotationContent::MathMl(math.into()),
            attributes: Default::default(),
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
