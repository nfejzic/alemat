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

pub trait Render {
    type Output;

    fn render_action(&mut self, action: &Action) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", action);
    }

    fn render_annotation(&mut self, annotation: &Annotation) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", annotation);
    }

    fn render_error(&mut self, error: &Error) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", error);
    }

    fn render_frac(&mut self, frac: &Frac) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", frac);
    }

    fn render_ident(&mut self, ident: &Ident) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", ident);
    }

    fn render_multiscripts(&mut self, multiscripts: &Multiscripts) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", multiscripts);
    }

    fn render_prescripts(&mut self, prescripts: &Prescripts) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", prescripts);
    }

    fn render_num(&mut self, num: &Num) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", num);
    }

    fn render_operator(&mut self, operator: &Operator) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", operator);
    }

    fn render_padded(&mut self, padded: &Padded) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", padded);
    }

    fn render_phantom(&mut self, phantom: &Phantom) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", phantom);
    }

    fn render_radical(&mut self, radical: &Radical) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", radical);
    }

    fn render_row(&mut self, row: &Row) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", row);
    }

    fn render_semantics(&mut self, semantics: &Semantics) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", semantics);
    }

    fn render_space(&mut self, space: &Space) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", space);
    }

    fn render_str_literal(&mut self, str_literal: &StrLiteral) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", str_literal);
    }

    fn render_style(&mut self, style: &Style) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", style);
    }

    fn render_subsup(&mut self, sub_sup: &SubSup) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", sub_sup);
    }

    fn render_table(&mut self, table: &Table) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", table);
    }

    fn render_text(&mut self, text: &Text) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", text);
    }

    fn render_underover(&mut self, under_over: &UnderOver) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", under_over);
    }

    fn render_attr(&mut self, attr: &Attribute) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", attr);
    }

    fn render_element(&mut self, tag: &Element) -> Self::Output {
        match tag {
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

    fn render_mathml(&mut self, mathml: &MathMl) -> Self::Output {
        unimplemented!("Rendering of {:?} not implemented", mathml)
    }
}

pub trait Writer {
    type Buffer<'s>
    where
        Self: 's;

    fn write_action(&mut self, action: &Action) {
        unimplemented!("Rendering of {:?} not implemented", action);
    }

    fn write_annotation(&mut self, annotation: &Annotation) {
        unimplemented!("Rendering of {:?} not implemented", annotation);
    }

    fn write_error(&mut self, error: &Error) {
        unimplemented!("Rendering of {:?} not implemented", error);
    }

    fn write_frac(&mut self, frac: &Frac) {
        unimplemented!("Rendering of {:?} not implemented", frac);
    }

    fn write_ident(&mut self, ident: &Ident) {
        unimplemented!("Rendering of {:?} not implemented", ident);
    }

    fn write_multiscripts(&mut self, multiscripts: &Multiscripts) {
        unimplemented!("Rendering of {:?} not implemented", multiscripts);
    }

    fn write_prescripts(&mut self, prescripts: &Prescripts) {
        unimplemented!("Rendering of {:?} not implemented", prescripts);
    }

    fn write_num(&mut self, num: &Num) {
        unimplemented!("Rendering of {:?} not implemented", num);
    }

    fn write_operator(&mut self, operator: &Operator) {
        unimplemented!("Rendering of {:?} not implemented", operator);
    }

    fn write_padded(&mut self, padded: &Padded) {
        unimplemented!("Rendering of {:?} not implemented", padded);
    }

    fn write_phantom(&mut self, phantom: &Phantom) {
        unimplemented!("Rendering of {:?} not implemented", phantom);
    }

    fn write_radical(&mut self, radical: &Radical) {
        unimplemented!("Rendering of {:?} not implemented", radical);
    }

    fn write_row(&mut self, row: &Row) {
        unimplemented!("Rendering of {:?} not implemented", row);
    }

    fn write_semantics(&mut self, semantics: &Semantics) {
        unimplemented!("Rendering of {:?} not implemented", semantics);
    }

    fn write_space(&mut self, space: &Space) {
        unimplemented!("Rendering of {:?} not implemented", space);
    }

    fn write_str_literal(&mut self, str_literal: &StrLiteral) {
        unimplemented!("Rendering of {:?} not implemented", str_literal);
    }

    fn write_style(&mut self, style: &Style) {
        unimplemented!("Rendering of {:?} not implemented", style);
    }

    fn write_subsup(&mut self, sub_sup: &SubSup) {
        unimplemented!("Rendering of {:?} not implemented", sub_sup);
    }

    fn write_table(&mut self, table: &Table) {
        unimplemented!("Rendering of {:?} not implemented", table);
    }

    fn write_text(&mut self, text: &Text) {
        unimplemented!("Rendering of {:?} not implemented", text);
    }

    fn write_underover(&mut self, under_over: &UnderOver) {
        unimplemented!("Rendering of {:?} not implemented", under_over);
    }

    fn write_attr(&mut self, attr: &Attribute) {
        unimplemented!("Rendering of {:?} not implemented", attr);
    }

    fn write_element(&mut self, tag: &Element) {
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

    fn write_mathml(&mut self, mathml: &MathMl) {
        unimplemented!("Rendering of {:?} not implemented", mathml)
    }

    fn buffer(&self) -> Self::Buffer<'_>;
}
