//! Library for type-safe building of MathML.

pub mod attributes;
mod buf_writer;
mod default_renderer;
pub mod elements;
pub mod markers;
mod to_mathml;

use attributes::Attribute;
pub use default_renderer::MathMlFormatter;
pub(crate) use elements::element_from_type;
use elements::IntoElements;
pub use elements::{Element, Elements};
pub use to_mathml::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MathMlAttr {
    Display(String),
    AltText(String),

    Global(Attribute),
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MathMl {
    content: Elements,
    attr: Vec<MathMlAttr>,
    // TODO: decide what fields should go inside
}

impl MathMl {
    pub fn content(&self) -> &Elements {
        &self.content
    }

    pub fn with_content(content: impl IntoElements) -> Self {
        Self {
            content: content.into_elements(),
            attr: Default::default(),
        }
    }

    pub fn append_content(&mut self, content: impl IntoElements) {
        self.content.append(&mut content.into_elements());
    }

    pub fn add_attr(&mut self, attr: impl Into<MathMlAttr>) {
        self.attr.push(attr.into());
    }

    pub fn with_attr<I, A>(mut self, attr: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<MathMlAttr>,
    {
        self.attr.extend(attr.into_iter().map(Into::into));
        self
    }

    pub fn extend_attr<I, A>(&mut self, attr: I)
    where
        I: IntoIterator<Item = A>,
        A: Into<MathMlAttr>,
    {
        self.attr.extend(attr.into_iter().map(Into::into))
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn render<R: Render>(&self, renderer: &mut R) -> R::Output {
        renderer.render_mathml(self)
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
