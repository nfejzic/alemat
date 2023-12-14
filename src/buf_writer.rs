use std::borrow::Borrow;
use std::fmt::Write;

use crate::{
    attributes::{Attribute, Dir, ScriptLevel},
    elements::{
        grouping::{ActionAttr, Prescripts},
        scripted::UnderOverAttr,
        AnnotationAttr, AnnotationContent, FracAttr, Num, OperatorAttr, PaddedAttr, SpaceAttr,
        TableAttr, TableCellAttr,
    },
    DisplayAttr, Element, MathMlAttr, Renderer, Writer,
};

/// Default implementation of MathMl [`Writer`].
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BufMathMlWriter {
    buf: String,
}

impl BufMathMlWriter {
    fn write_elements(&mut self, elements: &[Element]) -> Result<(), <Self as Writer>::Error> {
        for e in elements {
            self.write_element(e)?;
        }

        Ok(())
    }
}

impl Write for BufMathMlWriter {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buf.write_str(s)
    }
}

impl Writer for BufMathMlWriter {
    type Buffer = String;
    type Error = std::fmt::Error;

    fn write_action(
        &mut self,
        action: &crate::elements::grouping::Action,
    ) -> Result<(), Self::Error> {
        self.write_str("<maction")?;

        for attr in action.attributes().iter() {
            self.write_str(" ")?;

            match attr {
                ActionAttr::Global(g_attr) => {
                    self.write_attr(g_attr)?;
                }
                ActionAttr::Selection(sel) => {
                    self.write_str("selection=\"")?;
                    self.write_str(sel)?;
                    self.write_str("\"")?;
                }
                ActionAttr::ActionType(at) => {
                    self.write_str("actiontype=\"")?;
                    self.write_str(at)?;
                    self.write_str("\"")?;
                }
            }
        }

        self.write_str(">")?;

        self.write_elements(action.content())?;

        self.write_str("</maction>")
    }

    fn write_annotation(
        &mut self,
        annotation: &crate::elements::Annotation,
    ) -> Result<(), Self::Error> {
        let write_attr = |w: &mut BufMathMlWriter| -> Result<(), Self::Error> {
            for attr in annotation.attributes() {
                w.write_str(" ")?;

                match attr {
                    AnnotationAttr::Global(ref g_attr) => w.write_attr(g_attr)?,
                    AnnotationAttr::Encoding(ref enc) => {
                        w.write_str("encoding=\"")?;
                        w.write_str(enc)?;
                        w.write_str("\"")?;
                    }
                }
            }

            Ok(())
        };

        match annotation.content() {
            AnnotationContent::Text(text) => {
                self.write_str("<annotation")?;
                write_attr(self)?;
                self.write_str(">")?;
                self.write_str(text)?;
                self.write_str("</annotation>")
            }
            AnnotationContent::Nested(elements) => {
                self.write_str("<annotation-xml")?;
                write_attr(self)?;
                self.write_str(">")?;
                self.write_elements(elements)?;
                self.write_str("</annotation-xml>")
            }
        }
    }

    fn write_error(&mut self, error: &crate::elements::grouping::Error) -> Result<(), Self::Error> {
        self.write_str("<merror")?;

        for attr in error.attributes().iter() {
            self.write_str(" ")?;
            self.write_attr(attr)?;
        }

        self.write_str(">")?;
        self.write_elements(error.content())?;
        self.write_str("</merror>")
    }

    fn write_frac(&mut self, frac: &crate::elements::Frac) -> Result<(), Self::Error> {
        self.write_str("<mfrac")?;

        for attr in frac.attributes().iter() {
            self.write_str(" ")?;

            match attr {
                FracAttr::Global(ga) => self.write_attr(ga)?,
                FracAttr::LineThickness(lt) => {
                    self.write_str("linethickness=\"")?;
                    self.write_str(lt)?;
                    self.write_str("\"")?;
                }
            }
        }

        self.write_str(">")?;
        self.write_elements(frac.num())?;
        self.write_elements(frac.denom())?;
        self.write_str("</mfrac>")
    }

    fn write_ident(&mut self, ident: &crate::elements::Ident) -> Result<(), Self::Error> {
        self.write_str("<mi")?;

        for attr in ident.attributes().iter() {
            self.write_str(" ")?;
            self.write_attr(attr)?;
        }

        self.write_str(">")?;
        self.write_str(ident.ident())?;

        self.write_str("</mi>")?;

        Ok(())
    }

    fn write_multiscripts(
        &mut self,
        multiscripts: &crate::elements::scripted::Multiscripts,
    ) -> Result<(), Self::Error> {
        self.write_str("<mmultiscripts")?;

        for attr in multiscripts.attributes().iter() {
            self.write_str(" ")?;
            self.write_attr(attr)?;
        }

        self.write_str(">")?;
        self.write_elements(multiscripts.content())?;
        self.write_str("</mmultiscripts>")?;

        Ok(())
    }

    fn write_prescripts(&mut self, prescripts: &Prescripts) -> Result<(), Self::Error> {
        self.write_str("<mprescripts")?;

        for attr in prescripts.attributes().iter() {
            self.write_str(" ")?;
            self.write_attr(attr)?;
        }

        self.write_str("/>")?;

        Ok(())
    }

    fn write_num(&mut self, num: &crate::elements::Num) -> Result<(), Self::Error> {
        self.write_str("<mn")?;

        for attr in num.attributes().iter() {
            self.write_str(" ")?;
            self.write_attr(attr)?;
        }

        self.write_str(">")?;
        self.write_str(num.num())?;
        self.write_str("</mn>")?;

        Ok(())
    }

    fn write_operator(&mut self, operator: &crate::elements::Operator) -> Result<(), Self::Error> {
        self.write_str("<mo")?;

        for attr in operator.attributes().iter() {
            self.write_str(" ")?;

            match attr {
                OperatorAttr::Global(ga) => self.write_attr(ga)?,
                OperatorAttr::Form(form) => write!(self, r#"form="{form}""#)?,
                OperatorAttr::Fence => write!(self, "fence=\"true\"")?,
                OperatorAttr::Separator => write!(self, "separator=\"true\"")?,
                OperatorAttr::LeftSpace(sp) => write!(self, r#"lspace="{sp}""#)?,
                OperatorAttr::RightSpace(sp) => write!(self, r#"rspace="{sp}""#)?,
                OperatorAttr::MaxSize(s) => write!(self, r#"maxsize="{s}""#)?,
                OperatorAttr::MinSize(s) => write!(self, r#"minsize="{s}""#)?,
                OperatorAttr::Stretchy => self.write_str("stretchy=\"true\"")?,
                OperatorAttr::Symmetric => self.write_str("symmetric=\"true\"")?,
                OperatorAttr::LargeOp => self.write_str("largeop=\"true\"")?,
                OperatorAttr::MovableLimits => self.write_str("movablelimits=\"true\"")?,
            }
        }

        self.write_str(">")?;
        self.write_str(operator.op())?;
        self.write_str("</mo>")?;

        Ok(())
    }

    fn write_padded(&mut self, padded: &crate::elements::Padded) -> Result<(), Self::Error> {
        self.write_str("<mpadded")?;

        for attr in padded.attributes().iter() {
            self.write_str(" ")?;

            match attr {
                PaddedAttr::Width(w) => write!(self, r#"width="{w}""#)?,
                PaddedAttr::Height(h) => write!(self, r#"height="{h}""#)?,
                PaddedAttr::Depth(d) => write!(self, r#"depth="{d}""#)?,
                PaddedAttr::LeftSpace(ls) => write!(self, r#"lspace="{ls}""#)?,
                PaddedAttr::VerticalOffset(voffs) => write!(self, r#"voffset="{voffs}""#)?,
                PaddedAttr::Global(ga) => self.write_attr(ga)?,
            }
        }

        self.write_str(">")?;
        self.write_elements(padded.children())?;
        self.write_str("</mpadded>")
    }

    fn write_phantom(
        &mut self,
        phantom: &crate::elements::grouping::Phantom,
    ) -> Result<(), Self::Error> {
        self.write_str("<mphantom")?;

        for attr in phantom.attributes().iter() {
            self.write_str(" ")?;
            self.write_attr(attr)?;
        }

        self.write_str(">")?;
        self.write_elements(phantom.children())?;
        self.write_str("</mphantom>")
    }

    fn write_radical(
        &mut self,
        radical: &crate::elements::radicals::Radical,
    ) -> Result<(), Self::Error> {
        if radical.is_square() {
            self.write_str("<msqrt")?;
        } else {
            self.write_str("<mroot")?;
        }

        for attr in radical.attributes().iter() {
            self.write_str(" ")?;
            self.write_attr(attr)?;
        }

        self.write_str(">")?;
        self.write_elements(radical.content())?;

        if radical.is_square() {
            self.write_str("</msqrt>")
        } else {
            self.write_elements(radical.index())?;
            self.write_str("</mroot>")
        }
    }

    fn write_row(&mut self, row: &crate::elements::grouping::Row) -> Result<(), Self::Error> {
        self.write_str("<mrow")?;

        for attr in row.attributes().iter() {
            self.write_str(" ")?;
            self.write_attr(attr)?;
        }

        self.write_str(">")?;
        self.write_elements(row.children())?;
        self.write_str("</mrow>")
    }

    fn write_semantics(
        &mut self,
        semantics: &crate::elements::Semantics,
    ) -> Result<(), Self::Error> {
        self.write_str("<semantics")?;

        for attr in semantics.attributes().iter() {
            self.write_str(" ")?;
            self.write_attr(attr)?;
        }

        self.write_str(">")?;
        self.write_elements(semantics.children())?;
        self.write_str("</semantics>")
    }

    fn write_space(&mut self, space: &crate::elements::Space) -> Result<(), Self::Error> {
        self.write_str("<mspace")?;

        for attr in space.attributes().iter() {
            self.write_str(" ")?;

            match attr {
                SpaceAttr::Width(w) => write!(self, r#"width="{w}""#)?,
                SpaceAttr::Height(h) => write!(self, r#"height="{h}""#)?,
                SpaceAttr::Depth(d) => write!(self, r#"depth="{d}""#)?,
                SpaceAttr::Global(ref ga) => self.write_attr(ga)?,
            }
        }

        self.write_str("/>")
    }

    fn write_str_literal(
        &mut self,
        str_literal: &crate::elements::StrLiteral,
    ) -> Result<(), Self::Error> {
        self.write_str("<ms")?;

        for attr in str_literal.attributes().iter() {
            self.write_str(" ")?;
            self.write_attr(attr)?;
        }

        self.write_str(">")?;
        self.write_str(str_literal.content())?;
        self.write_str("</ms>")
    }

    fn write_style(&mut self, style: &crate::elements::grouping::Style) -> Result<(), Self::Error> {
        self.write_str("<mstyle")?;

        for attr in style.attributes().iter() {
            self.write_str(" ")?;
            self.write_attr(attr)?;
        }

        self.write_str(">")?;
        self.write_elements(style.children())?;
        self.write_str("</mstyle>")
    }

    fn write_subsup(
        &mut self,
        sub_sup: &crate::elements::scripted::SubSup,
    ) -> Result<(), Self::Error> {
        let sub = sub_sup.sub();
        let sup = sub_sup.sup();

        let write_attr = |w: &mut BufMathMlWriter| -> Result<(), Self::Error> {
            for attr in sub_sup.attributes().iter() {
                w.write_str(" ")?;
                w.write_attr(attr)?;
            }

            Ok(())
        };

        match (sub, sup) {
            (None, None) => unreachable!("SubSup element must have at least one of sub or sup."),
            (None, Some(sup)) => {
                self.write_str("<msup")?;
                write_attr(self)?;
                self.write_str(">")?;
                self.write_elements(sub_sup.base())?;
                self.write_elements(sup)?;
                self.write_str("</msup>")
            }
            (Some(sub), None) => {
                self.write_str("<msub")?;
                write_attr(self)?;
                self.write_str(">")?;
                self.write_elements(sub_sup.base())?;
                self.write_elements(sub)?;
                self.write_str("</msub>")
            }
            (Some(sub), Some(sup)) => {
                self.write_str("<msubsup")?;
                write_attr(self)?;
                self.write_str(">")?;
                self.write_elements(sub_sup.base())?;
                self.write_elements(sub)?;
                self.write_elements(sup)?;
                self.write_str("</msubsup>")
            }
        }
    }

    fn write_table(&mut self, table: &crate::elements::Table) -> Result<(), Self::Error> {
        self.write_str("<mtable")?;

        for table_attr in table.attributes().iter() {
            self.write_str(" ")?;

            match table_attr {
                TableAttr::ColumnLines(cl) => {
                    self.write_str("columnlines=\"")?;
                    for line in cl.iter() {
                        self.write_str(line.as_ref())?;
                        self.write_str(" ")?;
                    }

                    self.write_str("\"")?;
                }
                TableAttr::Global(ref ga) => self.write_attr(ga)?,
            }
        }

        self.write_str(">")?;

        for row in table.rows() {
            self.write_str("<mtr")?;

            for row_attr in row.attributes().iter() {
                self.write_str(" ")?;
                self.write_attr(row_attr)?;
            }

            self.write_str(">")?;

            for cell in row.cells() {
                self.write_str("<mtd")?;

                for cell_attr in cell.attributes().iter() {
                    self.write_str(" ")?;

                    match cell_attr {
                        TableCellAttr::ColumnSpan(cs) => write!(self, r#"columnspan="{cs}""#)?,
                        TableCellAttr::RowSpan(rs) => write!(self, r#"rowspan="{rs}""#)?,
                        TableCellAttr::Global(ref ga) => self.write_attr(ga)?,
                    }
                }

                self.write_str(">")?;
                self.write_elements(cell.children())?;
                self.write_str("</mtd>")?;
            }

            self.write_str("</mtr>")?;
        }

        self.write_str("</mtable>")
    }

    fn write_text(&mut self, text: &crate::elements::Text) -> Result<(), Self::Error> {
        self.write_str("<mtext")?;

        for attr in text.attributes().iter() {
            self.write_str(" ")?;
            self.write_attr(attr)?;
        }

        self.write_str(">")?;
        self.write_str(text.text())?;
        self.write_str("</mtext>")
    }

    fn write_underover(
        &mut self,
        under_over: &crate::elements::scripted::UnderOver,
    ) -> Result<(), Self::Error> {
        let under = under_over.under();
        let over = under_over.over();

        let write_attr = |w: &mut BufMathMlWriter| -> Result<(), Self::Error> {
            w.write_str(" ")?;

            for attr in under_over.attributes().iter() {
                match attr {
                    UnderOverAttr::AccentUnder => w.write_str(r#"accentunder="true""#)?,
                    UnderOverAttr::AccentOver => w.write_str(r#"accent="true""#)?,
                    UnderOverAttr::Global(ref ga) => w.write_attr(ga)?,
                }
            }

            Ok(())
        };

        match (under, over) {
            (None, None) => unreachable!("SubSup element must have at least one of sub or sup."),
            (None, Some(over)) => {
                self.write_str("<mover")?;
                write_attr(self)?;
                self.write_str(">")?;
                self.write_elements(under_over.expr())?;
                self.write_elements(over)?;
                self.write_str("</mover>")
            }
            (Some(under), None) => {
                self.write_str("<munder")?;
                write_attr(self)?;
                self.write_str(">")?;
                self.write_elements(under_over.expr())?;
                self.write_elements(under)?;
                self.write_str("</munder>")
            }
            (Some(under), Some(over)) => {
                self.write_str("<munderover")?;
                write_attr(self)?;
                self.write_str(">")?;
                self.write_elements(under_over.expr())?;
                self.write_elements(under)?;
                self.write_elements(over)?;
                self.write_str("</munderover>")
            }
        }
    }

    fn write_attr(&mut self, attr: &crate::attributes::Attribute) -> Result<(), Self::Error> {
        match attr {
            Attribute::Class(c) => write!(self, "class=\"{}\"", c),
            Attribute::Data { name, value } => write!(self, r#"data-{name}="{value}""#),
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
            Attribute::Id(id) => write!(self, r#"id="{id}""#),
            Attribute::MathBackground(c) => write!(self, r#"mathbackground="{c}""#),
            Attribute::MathColor(c) => write!(self, r#"mathcolor="{c}""#),
            Attribute::MathSize(s) => write!(self, r#"mathsize="{s}""#),
            Attribute::Nonce(n) => write!(self, r#"nonce="{n}""#),
            Attribute::ScriptLevel(sl) => match sl {
                ScriptLevel::Add(num) => write!(self, r#"scriptlevel="+{}""#, num),
                ScriptLevel::Sub(num) => write!(self, r#"scriptlevel="-{}""#, num),
                ScriptLevel::Num(num) => write!(self, r#"scriptlevel="{}""#, num),
            },
            Attribute::Style(st) => write!(self, r#"style="{st}""#),
            Attribute::TabIndex(ti) => write!(self, r#"tabindex="{ti}""#),
            Attribute::OnHandler { name, handler } => {
                write!(self, r#"on{name}="{handler}""#)
            }
            Attribute::MathVariant(mv) => write!(self, r#"mathvariant="{mv}""#),
        }
    }

    fn write_mathml(&mut self, mathml: &crate::MathMl) -> Result<(), Self::Error> {
        self.write_str("<math")?;

        for attr in mathml.attr.iter() {
            self.write_str(" ")?;

            match attr {
                MathMlAttr::Display(d) => {
                    write!(self, r#"display=""#)?;
                    match d {
                        DisplayAttr::Block => write!(self, r#"block""#)?,
                        DisplayAttr::Inline => write!(self, r#"inline""#)?,
                    }
                }
                MathMlAttr::AltText(alt_t) => write!(self, r#"alttext="{alt_t}""#)?,
                MathMlAttr::Global(a) => self.write_attr(a)?,
            }
        }
        self.write_str(">")?;
        self.write_elements(mathml.content())?;
        self.write_str("</math>")
    }

    fn buffer<T>(&self) -> &T
    where
        Self::Buffer: std::borrow::Borrow<T>,
    {
        self.buf.borrow()
    }

    fn into_inner(self) -> Self::Buffer {
        self.buf
    }

    fn finish(&mut self) -> Self::Buffer {
        std::mem::take(&mut self.buf)
    }
}

impl Renderer for BufMathMlWriter {
    type Output = String;
    type Error = std::fmt::Error;

    fn render_mathml(&mut self, mathml: &crate::MathMl) -> Result<Self::Output, Self::Error> {
        self.write_mathml(mathml)?;
        Ok(std::mem::take(&mut self.buf))
    }

    fn render_action(
        &mut self,
        action: &crate::elements::grouping::Action,
    ) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_action(action)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }

    fn render_annotation(
        &mut self,
        annotation: &crate::elements::Annotation,
    ) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_annotation(annotation)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }

    fn render_error(
        &mut self,
        error: &crate::elements::grouping::Error,
    ) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_error(error)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }

    fn render_frac(&mut self, frac: &crate::elements::Frac) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_frac(frac)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }

    fn render_ident(
        &mut self,
        ident: &crate::elements::Ident,
    ) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_ident(ident)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }

    fn render_multiscripts(
        &mut self,
        multiscripts: &crate::elements::scripted::Multiscripts,
    ) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_multiscripts(multiscripts)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }

    fn render_prescripts(&mut self, prescripts: &Prescripts) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_prescripts(prescripts)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }

    fn render_num(&mut self, num: &Num) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_num(num)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }

    fn render_operator(
        &mut self,
        operator: &crate::elements::Operator,
    ) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_operator(operator)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }

    fn render_padded(
        &mut self,
        padded: &crate::elements::Padded,
    ) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_padded(padded)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }

    fn render_phantom(
        &mut self,
        phantom: &crate::elements::grouping::Phantom,
    ) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_phantom(phantom)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }

    fn render_radical(
        &mut self,
        radical: &crate::elements::radicals::Radical,
    ) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_radical(radical)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }

    fn render_row(
        &mut self,
        row: &crate::elements::grouping::Row,
    ) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_row(row)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }

    fn render_semantics(
        &mut self,
        semantics: &crate::elements::Semantics,
    ) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_semantics(semantics)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }

    fn render_space(
        &mut self,
        space: &crate::elements::Space,
    ) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_space(space)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }

    fn render_str_literal(
        &mut self,
        str_literal: &crate::elements::StrLiteral,
    ) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_str_literal(str_literal)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }

    fn render_style(
        &mut self,
        style: &crate::elements::grouping::Style,
    ) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_style(style)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }

    fn render_subsup(
        &mut self,
        sub_sup: &crate::elements::scripted::SubSup,
    ) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_subsup(sub_sup)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }

    fn render_table(
        &mut self,
        table: &crate::elements::Table,
    ) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_table(table)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }

    fn render_text(&mut self, text: &crate::elements::Text) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_text(text)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }

    fn render_underover(
        &mut self,
        under_over: &crate::elements::scripted::UnderOver,
    ) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_underover(under_over)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }

    fn render_attr(&mut self, attr: &Attribute) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_attr(attr)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }

    fn render_element(&mut self, element: &Element) -> Result<Self::Output, Self::Error> {
        let mut s = std::mem::take(&mut self.buf);
        self.write_element(element)?;
        std::mem::swap(&mut self.buf, &mut s);
        Ok(s)
    }
}
