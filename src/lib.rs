#![warn(missing_docs)]
//! Library for type-safe building of MathML (core) markup.

mod buf_writer;
mod default_renderer;
mod to_mathml;
use elements::IntoElements;

pub mod attributes;
pub mod elements;
pub mod markers;

pub use attributes::Attribute;
pub use buf_writer::BufMathMlWriter;
pub use default_renderer::MathMlFormatter;
pub use elements::{Element, Elements};
pub use to_mathml::*;

pub(crate) use elements::element_from_type;

/// Specifies how the enclosed MathML markup should be rendered.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DisplayAttr {
    /// `block` display attribute. The element will be displayed in its own block outside the
    /// current span of text and with math-style set to normal.
    Block,

    /// `inline` display attribute. The element will be displayed inside the current span of text
    /// and with math-style set to compact.
    Inline,
}

/// Attributes of the `math` ([`MathMl`]) element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MathMlAttr {
    /// This enumerated attribute specifies how the enclosed MathML markup should be rendered. It
    /// can have one of the following values:
    ///
    ///  * `block`, which means that this element will be displayed in its own block outside the
    ///    current span of text and with math-style set to normal.
    ///  * `inline`, which means that this element will be displayed inside the current span of
    ///    text and with math-style set to compact.
    ///
    /// If not present, its default value is `inline`.
    Display(DisplayAttr),

    /// The alttext attribute may be used as alternative text by some legacy systems that do not
    /// implement math layout.
    AltText(String),

    /// One of the global [`Attribute`]s.
    Global(Attribute),
}

/// The `MathMl` is the `math` - the top-level MathML element, used to write a single mathematical
/// formula. It can be placed in HTML content where flow content is permitted.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MathMl {
    /// Inner content of the `math` element.
    content: Elements,

    /// Attributes of the `math` element.
    attr: Vec<MathMlAttr>,
}

impl MathMl {
    /// Get a reference to the inner content of the `math` element.
    pub fn content(&self) -> &Elements {
        &self.content
    }

    /// Map the content contained in [`MathMl`].
    ///
    /// Useful, for example, when wrapping the content in [`elements::Row`] is desired.
    ///
    /// # Example
    /// ```ignore
    /// let out = MathMl::with_content(
    ///     Frac::builder()
    ///         .num(Ident::from("x"))
    ///         .denom(Ident::from("y"))
    ///         .build(),
    /// )
    /// .map(Row::from)
    /// .render();
    /// ```
    pub fn map<T>(mut self, f: impl FnOnce(Elements) -> T) -> Self
    where
        T: IntoElements,
    {
        self.content = f(self.content).into_elements();
        self
    }

    /// Get a reference to all attributes of the `math` element.
    pub fn attributes(&self) -> &[MathMlAttr] {
        &self.attr
    }

    /// Create a new `math` element with the given content.
    pub fn with_content(content: impl IntoElements) -> Self {
        Self {
            content: content.into_elements(),
            attr: Default::default(),
        }
    }

    /// Append more content to this `math` element.
    pub fn append_content(&mut self, content: impl IntoElements) {
        self.content.append(&mut content.into_elements());
    }

    /// Add a single attribute to this `math` element.
    pub fn add_attr(&mut self, attr: impl Into<MathMlAttr>) {
        self.attr.push(attr.into());
    }

    /// Create new instance of `MathMl` with additional attributes.
    pub fn with_attr<I, A>(mut self, attr: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<MathMlAttr>,
    {
        self.attr.extend(attr.into_iter().map(Into::into));
        self
    }

    /// Extend the attributes of this `math` element.
    pub fn extend_attr<I, A>(&mut self, attr: I)
    where
        I: IntoIterator<Item = A>,
        A: Into<MathMlAttr>,
    {
        self.attr.extend(attr.into_iter().map(Into::into))
    }

    /// Return the number of elements this `math` element contains.
    pub fn len(&self) -> usize {
        self.content.len()
    }

    /// Check whether this `math` element contains no elements.
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Render this `math` element and its children using the given renderer.
    pub fn render_with<R: Renderer>(&self, renderer: &mut R) -> Result<R::Output, R::Error> {
        renderer.render_mathml(self)
    }

    /// Write this `math` element and its children using the given writer.
    pub fn write<'writer, W: Writer>(
        &self,
        writer: &'writer mut W,
    ) -> Result<&'writer mut W, W::Error> {
        writer.write_mathml(self)?;
        Ok(writer)
    }

    /// Render this `math` element and its children using the default renderer.
    ///
    /// In this implementation, [`BufMathMlWriter`] is used.
    pub fn render(&self) -> Result<String, <BufMathMlWriter as crate::Writer>::Error> {
        self.write(&mut BufMathMlWriter::default())
            .map(|w| w.finish())
    }
}

macro_rules! from_types {
    ($($type:path),* $(,)? => $for_type:path; $func:expr) => {
        $(
        impl From<$type> for $for_type {
            fn from(value: $type) -> Self {
                let from_value = $func;
                from_value(value)
            }
        }
        )*
    };
}

pub(crate) use from_types;
