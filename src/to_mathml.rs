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

pub trait MathMlRenderer {
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
