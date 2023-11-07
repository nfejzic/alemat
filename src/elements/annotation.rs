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
