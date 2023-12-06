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

pub trait Renderer {
    type Output;
    type Error;

    fn render_action(&mut self, action: &Action) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", action);
    }

    fn render_annotation(&mut self, annotation: &Annotation) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", annotation);
    }

    fn render_error(&mut self, error: &Error) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", error);
    }

    fn render_frac(&mut self, frac: &Frac) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", frac);
    }

    fn render_ident(&mut self, ident: &Ident) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", ident);
    }

    fn render_multiscripts(
        &mut self,
        multiscripts: &Multiscripts,
    ) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", multiscripts);
    }

    fn render_prescripts(&mut self, prescripts: &Prescripts) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", prescripts);
    }

    fn render_num(&mut self, num: &Num) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", num);
    }

    fn render_operator(&mut self, operator: &Operator) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", operator);
    }

    fn render_padded(&mut self, padded: &Padded) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", padded);
    }

    fn render_phantom(&mut self, phantom: &Phantom) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", phantom);
    }

    fn render_radical(&mut self, radical: &Radical) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", radical);
    }

    fn render_row(&mut self, row: &Row) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", row);
    }

    fn render_semantics(&mut self, semantics: &Semantics) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", semantics);
    }

    fn render_space(&mut self, space: &Space) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", space);
    }

    fn render_str_literal(
        &mut self,
        str_literal: &StrLiteral,
    ) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", str_literal);
    }

    fn render_style(&mut self, style: &Style) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", style);
    }

    fn render_subsup(&mut self, sub_sup: &SubSup) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", sub_sup);
    }

    fn render_table(&mut self, table: &Table) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", table);
    }

    fn render_text(&mut self, text: &Text) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", text);
    }

    fn render_underover(&mut self, under_over: &UnderOver) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", under_over);
    }

    fn render_attr(&mut self, attr: &Attribute) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", attr);
    }

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

    fn render_mathml(&mut self, mathml: &MathMl) -> Result<Self::Output, Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", mathml)
    }
}

pub trait Writer {
    type Buffer;
    type Error;

    fn write_action(&mut self, action: &Action) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", action);
    }

    fn write_annotation(&mut self, annotation: &Annotation) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", annotation);
    }

    fn write_error(&mut self, error: &Error) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", error);
    }

    fn write_frac(&mut self, frac: &Frac) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", frac);
    }

    fn write_ident(&mut self, ident: &Ident) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", ident);
    }

    fn write_multiscripts(&mut self, multiscripts: &Multiscripts) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", multiscripts);
    }

    fn write_prescripts(&mut self, prescripts: &Prescripts) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", prescripts);
    }

    fn write_num(&mut self, num: &Num) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", num);
    }

    fn write_operator(&mut self, operator: &Operator) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", operator);
    }

    fn write_padded(&mut self, padded: &Padded) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", padded);
    }

    fn write_phantom(&mut self, phantom: &Phantom) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", phantom);
    }

    fn write_radical(&mut self, radical: &Radical) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", radical);
    }

    fn write_row(&mut self, row: &Row) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", row);
    }

    fn write_semantics(&mut self, semantics: &Semantics) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", semantics);
    }

    fn write_space(&mut self, space: &Space) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", space);
    }

    fn write_str_literal(&mut self, str_literal: &StrLiteral) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", str_literal);
    }

    fn write_style(&mut self, style: &Style) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", style);
    }

    fn write_subsup(&mut self, sub_sup: &SubSup) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", sub_sup);
    }

    fn write_table(&mut self, table: &Table) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", table);
    }

    fn write_text(&mut self, text: &Text) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", text);
    }

    fn write_underover(&mut self, under_over: &UnderOver) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", under_over);
    }

    fn write_attr(&mut self, attr: &Attribute) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", attr);
    }

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

    fn write_mathml(&mut self, mathml: &MathMl) -> Result<(), Self::Error> {
        unimplemented!("Rendering of {:?} not implemented", mathml)
    }

    fn buffer<T>(&self) -> &T
    where
        Self::Buffer: Borrow<T>;

    fn into_inner(self) -> Self::Buffer;
}
