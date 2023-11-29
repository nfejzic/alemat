use crate::{
    attributes::Attribute,
    elements::{
        grouping::{Action, Error, Phantom, Row, Style},
        radicals::Radical,
        scripted::{Multiscripts, SubSup, UnderOver},
        Annotation, Frac, Ident, Num, Operator, Padded, Semantics, Space, StrLiteral, Table, Text,
    },
    Element, MathMl,
};

pub trait MathMlRenderer {
    fn render_action(&self, action: &Action) -> String {
        unimplemented!("Rendering of {:?} not implemented", action);
    }

    fn render_annotation(&self, annotation: &Annotation) -> String {
        unimplemented!("Rendering of {:?} not implemented", annotation);
    }

    fn render_error(&self, error: &Error) -> String {
        unimplemented!("Rendering of {:?} not implemented", error);
    }

    fn render_frac(&self, frac: &Frac) -> String {
        unimplemented!("Rendering of {:?} not implemented", frac);
    }

    fn render_ident(&self, ident: &Ident) -> String {
        unimplemented!("Rendering of {:?} not implemented", ident);
    }

    fn render_multiscripts(&self, multiscripts: &Multiscripts) -> String {
        unimplemented!("Rendering of {:?} not implemented", multiscripts);
    }

    fn render_num(&self, num: &Num) -> String {
        unimplemented!("Rendering of {:?} not implemented", num);
    }

    fn render_operator(&self, operator: &Operator) -> String {
        unimplemented!("Rendering of {:?} not implemented", operator);
    }

    fn render_padded(&self, padded: &Padded) -> String {
        unimplemented!("Rendering of {:?} not implemented", padded);
    }

    fn render_phantom(&self, phantom: &Phantom) -> String {
        unimplemented!("Rendering of {:?} not implemented", phantom);
    }

    fn render_radical(&self, radical: &Radical) -> String {
        unimplemented!("Rendering of {:?} not implemented", radical);
    }

    fn render_row(&self, row: &Row) -> String {
        unimplemented!("Rendering of {:?} not implemented", row);
    }

    fn render_semantics(&self, semantics: &Semantics) -> String {
        unimplemented!("Rendering of {:?} not implemented", semantics);
    }

    fn render_space(&self, space: &Space) -> String {
        unimplemented!("Rendering of {:?} not implemented", space);
    }

    fn render_str_literal(&self, str_literal: &StrLiteral) -> String {
        unimplemented!("Rendering of {:?} not implemented", str_literal);
    }

    fn render_style(&self, style: &Style) -> String {
        unimplemented!("Rendering of {:?} not implemented", style);
    }

    fn render_subsup(&self, sub_sup: &SubSup) -> String {
        unimplemented!("Rendering of {:?} not implemented", sub_sup);
    }

    fn render_table(&self, table: &Table) -> String {
        unimplemented!("Rendering of {:?} not implemented", table);
    }

    fn render_text(&self, text: &Text) -> String {
        unimplemented!("Rendering of {:?} not implemented", text);
    }

    fn render_underover(&self, under_over: &UnderOver) -> String {
        unimplemented!("Rendering of {:?} not implemented", under_over);
    }

    fn render_attr(&self, attr: &Attribute) -> String {
        unimplemented!("Rendering of {:?} not implemented", attr);
    }

    fn render_element(&self, tag: &Element) -> String {
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
        }
    }

    fn render_mathml(&self, mathml: &MathMl) -> String {
        unimplemented!("Rendering of {:?} not implemented", mathml)
    }
}
