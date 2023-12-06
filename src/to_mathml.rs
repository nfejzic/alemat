//! This module provides traits for rendering MathML elements.

use std::borrow::Borrow;

use crate::{
    attributes::Attribute,
    elements::{
        grouping::{Action, Error, Phantom, Prescripts, Row, Style},
        radicals::Radical,
        scripted::{Multiscripts, SubSup, UnderOver},
        Annotation, Frac, Ident, Num, Operator, Padded, Semantics, Space, StrLiteral, Table, Text,
    },
    Element, MathMl,
};

/// Trait for rendering MathML elements. Each method in this trait has a default implementation
/// that returns [`Result::Ok`] with an empty [`String`].
/// This makes it possible to potentially introduce new elements without breaking existing
/// renderers.
/// This means that any new elements introduced will be omitted in the final render, if the
/// corresponding methods are not implemented.
pub trait Renderer {
    /// The `Output` type of the renderer that will be returned by `render_*` methods.
    type Output;

    /// The `Error` type of the renderer that will be returned by `render_*` methods in cases where
    /// rendering can fail. Use [`Infallible`] for renderers that cannot fail.
    ///
    /// [`Infallible`]: std::convert::Infallible
    type Error;

    /// Render an [`Action`] element.
    fn render_action(&mut self, action: &Action) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", action);
    }

    /// Render an [`Annotation`] element.
    fn render_annotation(&mut self, annotation: &Annotation) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", annotation);
    }

    /// Render an [`Error`] element.
    fn render_error(&mut self, error: &Error) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", error);
    }

    /// Render a [`Frac`] element.
    fn render_frac(&mut self, frac: &Frac) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", frac);
    }

    /// Render an [`Ident`] element.
    fn render_ident(&mut self, ident: &Ident) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", ident);
    }

    /// Render a [`Multiscripts`] element.
    fn render_multiscripts(
        &mut self,
        multiscripts: &Multiscripts,
    ) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", multiscripts);
    }

    /// Render a [`Prescripts`] element.
    fn render_prescripts(&mut self, prescripts: &Prescripts) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", prescripts);
    }

    /// Render a [`Num`] element.
    fn render_num(&mut self, num: &Num) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", num);
    }

    /// Render an [`Operator`] element.
    fn render_operator(&mut self, operator: &Operator) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", operator);
    }

    /// Render a [`Padded`] element.
    fn render_padded(&mut self, padded: &Padded) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", padded);
    }

    /// Render a [`Phantom`] element.
    fn render_phantom(&mut self, phantom: &Phantom) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", phantom);
    }

    /// Render a [`Radical`] element.
    fn render_radical(&mut self, radical: &Radical) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", radical);
    }

    /// Render a [`Row`] element.
    fn render_row(&mut self, row: &Row) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", row);
    }

    /// Render a [`Semantics`] element.
    fn render_semantics(&mut self, semantics: &Semantics) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", semantics);
    }

    /// Render a [`Space`] element.
    fn render_space(&mut self, space: &Space) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", space);
    }

    /// Render a [`StrLiteral`] element.
    fn render_str_literal(
        &mut self,
        str_literal: &StrLiteral,
    ) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", str_literal);
    }

    /// Render a [`Style`] element.
    fn render_style(&mut self, style: &Style) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", style);
    }

    /// Render a [`SubSup`] element.
    fn render_subsup(&mut self, sub_sup: &SubSup) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", sub_sup);
    }

    /// Render a [`Table`] element.
    fn render_table(&mut self, table: &Table) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", table);
    }

    /// Render a [`Text`] element.
    fn render_text(&mut self, text: &Text) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", text);
    }

    /// Render an [`UnderOver`] element.
    fn render_underover(&mut self, under_over: &UnderOver) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", under_over);
    }

    /// Render an [`Attribute`] element.
    fn render_attr(&mut self, attr: &Attribute) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", attr);
    }

    /// Render an [`Element`] element.
    fn render_element(&mut self, element: &Element) -> Result<Self::Output, Self::Error> {
        match element {
            Element::Action(action) => self.render_action(action),
            Element::Annotation(annotation) => self.render_annotation(annotation),
            Element::Error(error) => self.render_error(error),
            Element::Frac(frac) => self.render_frac(frac),
            Element::Ident(ident) => self.render_ident(ident),
            Element::Multiscripts(multiscripts) => self.render_multiscripts(multiscripts),
            Element::Num(num) => self.render_num(num),
            Element::Operator(operator) => self.render_operator(operator),
            Element::Padded(padded) => self.render_padded(padded),
            Element::Phantom(phantom) => self.render_phantom(phantom),
            Element::Radical(radical) => self.render_radical(radical),
            Element::Row(row) => self.render_row(row),
            Element::Semantics(semantics) => self.render_semantics(semantics),
            Element::Space(space) => self.render_space(space),
            Element::StrLiteral(str_literal) => self.render_str_literal(str_literal),
            Element::Style(style) => self.render_style(style),
            Element::SubSup(subsup) => self.render_subsup(subsup),
            Element::Table(table) => self.render_table(table),
            Element::Text(text) => self.render_text(text),
            Element::UnderOver(underover) => self.render_underover(underover),
            Element::Prescripts(prescripts) => self.render_prescripts(prescripts),
        }
    }

    /// Render a [`MathMl`] element.
    fn render_mathml(&mut self, mathml: &MathMl) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", mathml)
    }
}

/// Trait for writing MathML elements. Each method in this trait has a default implementation that
/// does nothing.
///
/// In contrast with [`Renderer`] trait, implementors of this trait are meant to write the rendered
/// representation into a buffer instead of returning the rendered output from each function.
/// This enables buffered rendering, where instead of allocating many [`String`]s (for example),
/// one [`String`] is allocated and written into.
///
/// Each method in this trait has a default implementation that corresponds to a no-op.
/// This makes it possible to potentially introduce new elements without breaking existing
/// renderers.
/// This also means that any new elements introduced will be omitted in the final render, as long
/// as the corresponding methods are not implemented.
pub trait Writer {
    /// The type of the buffer that this writer writes into.
    type Buffer;

    /// The `Error` type of the writer that will be returned by `write_*` methods in cases where
    /// writing can fail. Use [`Infallible`] for writers that cannot fail.
    ///
    /// [`Infallible`]: std::convert::Infallible
    type Error;

    /// Write an [`Action`] element.
    fn write_action(&mut self, _action: &Action) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write an [`Annotation`] element.
    fn write_annotation(&mut self, _annotation: &Annotation) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write an [`Error`] element.
    fn write_error(&mut self, _error: &Error) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write a [`Frac`] element.
    fn write_frac(&mut self, _frac: &Frac) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write an [`Ident`] element.
    fn write_ident(&mut self, _ident: &Ident) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write a [`Multiscripts`] element.
    fn write_multiscripts(&mut self, _multiscripts: &Multiscripts) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write a [`Prescripts`] element.
    fn write_prescripts(&mut self, _prescripts: &Prescripts) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write a [`Num`] element.
    fn write_num(&mut self, _num: &Num) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write an [`Operator`] element.
    fn write_operator(&mut self, _operator: &Operator) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write a [`Padded`] element.
    fn write_padded(&mut self, _padded: &Padded) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write a [`Phantom`] element.
    fn write_phantom(&mut self, _phantom: &Phantom) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write a [`Radical`] element.
    fn write_radical(&mut self, _radical: &Radical) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write a [`Row`] element.
    fn write_row(&mut self, _row: &Row) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write a [`Semantics`] element.
    fn write_semantics(&mut self, _semantics: &Semantics) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write a [`Space`] element.
    fn write_space(&mut self, _space: &Space) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write a [`StrLiteral`] element.
    fn write_str_literal(&mut self, _str_literal: &StrLiteral) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write a [`Style`] element.
    fn write_style(&mut self, _style: &Style) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write a [`SubSup`] element.
    fn write_subsup(&mut self, _sub_sup: &SubSup) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write a [`Table`] element.
    fn write_table(&mut self, _table: &Table) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write a [`Text`] element.
    fn write_text(&mut self, _text: &Text) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write an [`UnderOver`] element.
    fn write_underover(&mut self, _under_over: &UnderOver) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write an [`Attribute`] element.
    fn write_attr(&mut self, _attr: &Attribute) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Write an [`Element`] into the `Self::Buffer`.
    fn write_element(&mut self, tag: &Element) -> Result<(), Self::Error> {
        match tag {
            Element::Action(action) => self.write_action(action),
            Element::Annotation(annotation) => self.write_annotation(annotation),
            Element::Error(error) => self.write_error(error),
            Element::Frac(frac) => self.write_frac(frac),
            Element::Ident(ident) => self.write_ident(ident),
            Element::Multiscripts(multiscripts) => self.write_multiscripts(multiscripts),
            Element::Num(num) => self.write_num(num),
            Element::Operator(operator) => self.write_operator(operator),
            Element::Padded(padded) => self.write_padded(padded),
            Element::Phantom(phantom) => self.write_phantom(phantom),
            Element::Radical(radical) => self.write_radical(radical),
            Element::Row(row) => self.write_row(row),
            Element::Semantics(semantics) => self.write_semantics(semantics),
            Element::Space(space) => self.write_space(space),
            Element::StrLiteral(str_literal) => self.write_str_literal(str_literal),
            Element::Style(style) => self.write_style(style),
            Element::SubSup(subsup) => self.write_subsup(subsup),
            Element::Table(table) => self.write_table(table),
            Element::Text(text) => self.write_text(text),
            Element::UnderOver(underover) => self.write_underover(underover),
            Element::Prescripts(prescripts) => self.write_prescripts(prescripts),
        }
    }

    /// Write a [`MathMl`] element.
    fn write_mathml(&mut self, mathml: &MathMl) -> Result<(), Self::Error> {
        for element in mathml.content().iter() {
            self.write_element(element)?;
        }

        Ok(())
    }

    /// Return a reference to the buffer.
    fn buffer<T>(&self) -> &T
    where
        Self::Buffer: Borrow<T>;

    /// Produce a rendered representation of the [`MathMl`] content written into the [`Writer`].
    fn finish(&mut self) -> Self::Buffer;

    /// Consume the [`Writer`] and return the inner buffer.
    fn into_inner(self) -> Self::Buffer;
}
