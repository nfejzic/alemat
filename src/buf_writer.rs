use std::fmt::Write;

use crate::{
    attributes::{Attribute, Dir, ScriptLevel},
    elements::{
        grouping::{ActionAttr, Prescripts},
        scripted::UnderOverAttr,
        AnnotationAttr, AnnotationContent, FracAttr, Num, OperatorAttr, PaddedAttr, SpaceAttr,
        TableAttr, TableCellAttr,
    },
    Element, MathMlAttr, Writer,
};

pub struct BufMathMlWriter {
    buf: String,
}

impl BufMathMlWriter {
    fn write_elements(&mut self, elements: &[Element]) {
        for e in elements {
            self.write_element(e);
        }
    }

    fn write_str(&mut self, s: &str) {
        Write::write_str(self, s).unwrap();
    }
}

impl Write for BufMathMlWriter {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buf.write_str(s)
    }
}

impl Writer for BufMathMlWriter {
    type Buffer<'s> = &'s str;

    fn buffer(&self) -> Self::Buffer<'_> {
        self.buf.as_str()
    }

    fn write_action(&mut self, action: &crate::elements::grouping::Action) {
        self.write_str("<maction ");

        for attr in action.attributes().iter() {
            match attr {
                ActionAttr::Global(g_attr) => self.write_attr(g_attr),
                ActionAttr::Selection(sel) => {
                    self.write_str("selection=\"");
                    self.write_str(sel);
                    self.write_str("\"");
                }
                ActionAttr::ActionType(at) => {
                    self.write_str("actiontype=\"");
                    self.write_str(at);
                    self.write_str("\"");
                }
            }
        }

        self.write_str(">");

        self.write_elements(action.content());

        self.write_str("</maction>");
    }

    fn write_annotation(&mut self, annotation: &crate::elements::Annotation) {
        let write_attr = |w: &mut BufMathMlWriter| {
            for attr in annotation.attributes() {
                match attr {
                    AnnotationAttr::Global(ref g_attr) => w.write_attr(g_attr),
                    AnnotationAttr::Encoding(ref enc) => {
                        w.write_str("encoding=\"");
                        w.write_str(enc);
                        w.write_str("\"");
                    }
                }
            }
        };

        match annotation.content() {
            AnnotationContent::Text(text) => {
                self.write_str("<annotation ");
                write_attr(self);
                self.write_str(">");
                self.write_str(text);
                self.write_str("</annotation>");
            }
            AnnotationContent::Nested(elements) => {
                self.write_str("<annotation-xml ");
                write_attr(self);
                self.write_str(">");
                self.write_elements(elements);
                self.write_str("</annotation-xml>");
            }
        }
    }

    fn write_error(&mut self, error: &crate::elements::grouping::Error) {
        self.write_str("<merror ");

        for attr in error.attributes().iter() {
            self.write_attr(attr);
        }

        self.write_str(">");
        self.write_elements(error.content());
        self.write_str("</merror>");
    }

    fn write_frac(&mut self, frac: &crate::elements::Frac) {
        self.write_str("<mfrac ");

        for attr in frac.attributes().iter() {
            match attr {
                FracAttr::Global(ga) => self.write_attr(ga),
                FracAttr::LineThickness(lt) => {
                    self.write_str("linethickness=\"");
                    self.write_str(lt);
                    self.write_str("\"");
                }
            }
        }

        self.write_str(">");
        self.write_elements(frac.num());
        self.write_elements(frac.denom());
        self.write_str("</mfrac>");
    }

    fn write_ident(&mut self, ident: &crate::elements::Ident) {
        self.write_str("<mi ");

        for attr in ident.attributes().iter() {
            self.write_attr(attr);
        }

        self.write_str(">");
        self.write_str(ident.ident());

        self.write_str("</mi>");
    }

    fn write_multiscripts(&mut self, multiscripts: &crate::elements::scripted::Multiscripts) {
        self.write_str("<mmultiscripts ");

        for attr in multiscripts.attributes().iter() {
            self.write_attr(attr);
        }

        self.write_str(">");
        self.write_elements(multiscripts.content());
        self.write_str("</mmultiscripts>");
    }

    fn write_prescripts(&mut self, prescripts: &Prescripts) {
        self.write_str("<mprescripts ");

        for attr in prescripts.attributes().iter() {
            self.write_attr(attr);
        }

        self.write_str("/>");
    }

    fn write_num(&mut self, num: &crate::elements::Num) {
        self.write_str("<mn ");

        for attr in num.attributes().iter() {
            self.write_attr(attr);
        }

        self.write_str(">");
        self.write_str(num.num());
        self.write_str("</mn>");
    }

    fn write_operator(&mut self, operator: &crate::elements::Operator) {
        self.write_str("<mo ");

        for attr in operator.attributes().iter() {
            match attr {
                OperatorAttr::Global(ga) => self.write_attr(ga),
                OperatorAttr::Form(form) => write!(self, r#"form="{form}""#).unwrap(),
                OperatorAttr::Fence => write!(self, "fence=\"true\"").unwrap(),
                OperatorAttr::Separator => write!(self, "separator=\"true\"").unwrap(),
                OperatorAttr::LeftSpace(sp) => write!(self, r#"lspace="{sp}""#).unwrap(),
                OperatorAttr::RightSpace(sp) => write!(self, r#"rspace="{sp}""#).unwrap(),
                OperatorAttr::MaxSize(s) => write!(self, r#"maxsize="{s}""#).unwrap(),
                OperatorAttr::MinSize(s) => write!(self, r#"minsize="{s}""#).unwrap(),
                OperatorAttr::Stretchy => self.write_str("stretchy=\"true\""),
                OperatorAttr::Symmetric => self.write_str("symmetric=\"true\""),
                OperatorAttr::LargeOp => self.write_str("largeop=\"true\""),
                OperatorAttr::MovableLimits => self.write_str("movablelimits=\"true\""),
            }
        }

        self.write_str(">");
        self.write_str(operator.op());
        self.write_str("</mo>");
    }

    fn write_padded(&mut self, padded: &crate::elements::Padded) {
        self.write_str("<mpadded ");

        for attr in padded.attributes().iter() {
            match attr {
                PaddedAttr::Width(w) => write!(self, r#"width="{w}""#).unwrap(),
                PaddedAttr::Height(h) => write!(self, r#"height="{h}""#).unwrap(),
                PaddedAttr::Depth(d) => write!(self, r#"depth="{d}""#).unwrap(),
                PaddedAttr::LeftSpace(ls) => write!(self, r#"lspace="{ls}""#).unwrap(),
                PaddedAttr::VerticalOffset(voffs) => write!(self, r#"voffset="{voffs}""#).unwrap(),
                PaddedAttr::Global(ga) => self.write_attr(ga),
            }
        }

        self.write_str(">");
        self.write_elements(padded.children());
        self.write_str("</mpadded>");
    }

    fn write_phantom(&mut self, phantom: &crate::elements::grouping::Phantom) {
        self.write_str("<mphantom ");

        for attr in phantom.attributes().iter() {
            self.write_attr(attr);
        }

        self.write_str(">");
        self.write_elements(phantom.children());
        self.write_str("</mphantom>");
    }

    fn write_radical(&mut self, radical: &crate::elements::radicals::Radical) {
        if radical.is_square() {
            self.write_str("<msqrt ");
        } else {
            self.write_str("<mroot ");
        }

        for attr in radical.attributes().iter() {
            self.write_attr(attr);
        }

        self.write_str(">");
        self.write_elements(radical.content());

        if radical.is_square() {
            self.write_str("</msqrt>");
        } else {
            self.write_num(&Num::from(radical.index()));
            self.write_str("</mroot>");
        }
    }

    fn write_row(&mut self, row: &crate::elements::grouping::Row) {
        self.write_str("<mrow ");

        for attr in row.attributes().iter() {
            self.write_attr(attr)
        }

        self.write_str(">");
        self.write_elements(row.children());
        self.write_str("</mrow>");
    }

    fn write_semantics(&mut self, semantics: &crate::elements::Semantics) {
        self.write_str("<semantics ");

        for attr in semantics.attributes().iter() {
            self.write_attr(attr);
        }

        self.write_str(">");
        self.write_elements(semantics.children());
        self.write_str("</semantics>");
    }

    fn write_space(&mut self, space: &crate::elements::Space) {
        self.write_str("<mspace ");

        for attr in space.attributes().iter() {
            match attr {
                SpaceAttr::Width(w) => write!(self, r#"width="{w}""#).unwrap(),
                SpaceAttr::Height(h) => write!(self, r#"height="{h}""#).unwrap(),
                SpaceAttr::Depth(d) => write!(self, r#"depth="{d}""#).unwrap(),
                SpaceAttr::Global(ref ga) => self.write_attr(ga),
            }
        }

        self.write_str("/>");
    }

    fn write_str_literal(&mut self, str_literal: &crate::elements::StrLiteral) {
        self.write_str("<ms ");

        for attr in str_literal.attributes().iter() {
            self.write_attr(attr);
        }

        self.write_str(">");
        self.write_str(str_literal.content());
        self.write_str("</ms>");
    }

    fn write_style(&mut self, style: &crate::elements::grouping::Style) {
        self.write_str("<mstyle ");

        for attr in style.attributes().iter() {
            self.write_attr(attr);
        }

        self.write_str(">");
        self.write_elements(style.children());
        self.write_str("</mstyle>");
    }

    fn write_subsup(&mut self, sub_sup: &crate::elements::scripted::SubSup) {
        let sub = sub_sup.sub();
        let sup = sub_sup.sup();

        let write_attr = |w: &mut BufMathMlWriter| {
            for attr in sub_sup.attributes().iter() {
                w.write_attr(attr);
            }
        };

        match (sub, sup) {
            (None, None) => unreachable!("SubSup element must have at least one of sub or sup."),
            (None, Some(sup)) => {
                self.write_str("<msup ");
                write_attr(self);
                self.write_str(">");
                self.write_elements(sub_sup.base());
                self.write_elements(sup);
                self.write_str("</msup>");
            }
            (Some(sub), None) => {
                self.write_str("<msub ");
                write_attr(self);
                self.write_str(">");
                self.write_elements(sub_sup.base());
                self.write_elements(sub);
                self.write_str("</msub>");
            }
            (Some(sub), Some(sup)) => {
                self.write_str("<msubsup ");
                write_attr(self);
                self.write_str(">");
                self.write_elements(sub_sup.base());
                self.write_elements(sub);
                self.write_elements(sup);
                self.write_str("</msubsup>");
            }
        }
    }

    fn write_table(&mut self, table: &crate::elements::Table) {
        self.write_str("<mtable ");

        for table_attr in table.attributes().iter() {
            match table_attr {
                TableAttr::ColumnLines(cl) => {
                    self.write_str("columnlines=\"");
                    for line in cl.iter() {
                        self.write_str(line.as_ref());
                        self.write_str(", ");
                    }

                    if !cl.is_empty() {
                        // remove the trailing comma
                        let _ = self.buf.pop();
                    }

                    self.write_str("\"");
                }
                TableAttr::Global(ref ga) => self.write_attr(ga),
            }
        }

        self.write_str(">");

        for row in table.rows() {
            self.write_str("<mtr ");

            for row_attr in row.attributes().iter() {
                self.write_attr(row_attr);
            }

            self.write_str(">");

            for cell in row.cells() {
                self.write_str("<mtd ");

                for cell_attr in cell.attributes().iter() {
                    match cell_attr {
                        TableCellAttr::ColumnSpan(cs) => {
                            write!(self, r#"columnspan="{cs}""#).unwrap()
                        }
                        TableCellAttr::RowSpan(rs) => write!(self, r#"rowspan="{rs}""#).unwrap(),
                        TableCellAttr::Global(ref ga) => self.write_attr(ga),
                    }
                }

                self.write_str(">");
                self.write_elements(cell.children());
                self.write_str("</mtd>");
            }

            self.write_str("</mtr>");
        }

        self.write_str("</mtable>");
    }

    fn write_text(&mut self, text: &crate::elements::Text) {
        self.write_str("<mtext ");

        for attr in text.attributes().iter() {
            self.write_attr(attr);
        }

        self.write_str(">");
        self.write_str(text.text());
        self.write_str("</mtext>");
    }

    fn write_underover(&mut self, under_over: &crate::elements::scripted::UnderOver) {
        let under = under_over.under();
        let over = under_over.over();

        let write_attr = |w: &mut BufMathMlWriter| {
            for attr in under_over.attributes().iter() {
                match attr {
                    UnderOverAttr::AccentUnder => w.write_str(r#"accentunder="true""#),
                    UnderOverAttr::AccentOver => w.write_str(r#"accent="true""#),
                    UnderOverAttr::Global(ref ga) => w.write_attr(ga),
                }
            }
        };

        match (under, over) {
            (None, None) => unreachable!("SubSup element must have at least one of sub or sup."),
            (None, Some(over)) => {
                self.write_str("<mover ");
                write_attr(self);
                self.write_str(">");
                self.write_elements(under_over.expr());
                self.write_elements(over);
                self.write_str("</mover>");
            }
            (Some(under), None) => {
                self.write_str("<munder ");
                write_attr(self);
                self.write_str(">");
                self.write_elements(under_over.expr());
                self.write_elements(under);
                self.write_str("</munder>");
            }
            (Some(under), Some(over)) => {
                self.write_str("<munderover ");
                write_attr(self);
                self.write_str(">");
                self.write_elements(under_over.expr());
                self.write_elements(under);
                self.write_elements(over);
                self.write_str("</munderover>");
            }
        }
    }

    fn write_attr(&mut self, attr: &crate::attributes::Attribute) {
        match attr {
            Attribute::Class(c) => write!(self, "class=\"{}\"", c).unwrap(),
            Attribute::Data { name, value } => write!(self, r#"data-{name}="{value}""#).unwrap(),
            Attribute::Dir(dir) => match dir {
                Dir::RightToLeft => self.write_str(r#"dir="rtl""#),
                Dir::LeftToRight => self.write_str(r#"dir="ltr""#),
            },
            Attribute::DisplayStyle(d) => {
                if *d {
                    self.write_str(r#"display="normal"#)
                } else {
                    self.write_str(r#"display="compact""#)
                }
            }
            Attribute::Id(id) => write!(self, r#"id="{id}""#).unwrap(),
            Attribute::MathBackground(c) => write!(self, r#"mathbackground="{c}""#).unwrap(),
            Attribute::MathColor(c) => write!(self, r#"mathcolor="{c}""#).unwrap(),
            Attribute::MathSize(s) => write!(self, r#"mathsize="{s}""#).unwrap(),
            Attribute::Nonce(n) => write!(self, r#"nonce="{n}""#).unwrap(),
            Attribute::ScriptLevel(sl) => match sl {
                ScriptLevel::Add(num) => write!(self, r#"scriptlevel="+{}""#, num).unwrap(),
                ScriptLevel::Sub(num) => write!(self, r#"scriptlevel="-{}""#, num).unwrap(),
                ScriptLevel::Num(num) => write!(self, r#"scriptlevel="{}""#, num).unwrap(),
            },
            Attribute::Style(st) => write!(self, r#"style="{st}""#).unwrap(),
            Attribute::TabIndex(ti) => write!(self, r#"tabindex="{ti}""#).unwrap(),
            Attribute::OnHandler { name, handler } => {
                write!(self, r#"on{name}="{handler}""#).unwrap()
            }
            Attribute::MathVariant(mv) => write!(self, r#"mathvariant="{mv}""#).unwrap(),
        }
    }

    fn write_mathml(&mut self, mathml: &crate::MathMl) {
        self.write_str("<math ");

        for attr in mathml.attr.iter() {
            match attr {
                MathMlAttr::Display(d) => write!(self, r#"display="{d}""#).unwrap(),
                MathMlAttr::AltText(alt_t) => write!(self, r#"alttext="{alt_t}""#).unwrap(),
                MathMlAttr::Global(a) => self.write_attr(a),
            }
        }
        self.write_str(">");
        self.write_elements(mathml.content());
        self.write_str("</math>");
    }
}
