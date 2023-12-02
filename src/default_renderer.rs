use crate::{
    attributes::{Attribute, Dir, ScriptLevel},
    elements::{
        grouping::{ActionAttr, Prescripts},
        scripted::UnderOverAttr,
        AnnotationAttr, AnnotationContent, FracAttr, Num, OperatorAttr, PaddedAttr, SpaceAttr,
        TableCellAttr,
    },
    Element, MathMlAttr, MathMlRenderer,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MathMlFormatter;

impl MathMlFormatter {
    fn render_elements(&self, elements: &[Element]) -> String {
        elements
            .iter()
            .map(|el| self.render_element(el))
            .collect::<String>()
    }
}

impl MathMlRenderer for MathMlFormatter {
    type Output = String;

    fn render_action(&self, action: &crate::elements::grouping::Action) -> Self::Output {
        let attributes = action
            .attributes()
            .iter()
            .map(|m| match m {
                ActionAttr::Global(g_attr) => self.render_attr(g_attr),
                ActionAttr::Selection(sel) => format!(r#"selection="{sel}""#),
                ActionAttr::ActionType(at) => format!(r#"actiontype="{at}""#),
            })
            .collect::<Vec<_>>()
            .join(" ");

        let content = self.render_elements(action.content());

        format!("<maction {attributes}>{content}</maction>")
    }

    fn render_annotation(&self, annotation: &crate::elements::Annotation) -> Self::Output {
        let (tag, content) = match annotation.content() {
            AnnotationContent::Text(ref t) => ("annotation", t.clone()),
            AnnotationContent::Nested(ref m) => ("annotation-xml", self.render_elements(m)),
        };

        let attrs = annotation
            .attributes()
            .iter()
            .map(|a| match a {
                AnnotationAttr::Global(g_attr) => self.render_attr(g_attr),
                AnnotationAttr::Encoding(enc) => format!(r#"encoding="{enc}""#),
            })
            .collect::<Vec<_>>()
            .join(" ");

        format!("<{tag} {attrs}>{content}</{tag}>")
    }

    fn render_error(&self, error: &crate::elements::grouping::Error) -> Self::Output {
        let content = self.render_elements(error.content());

        let attrs = error
            .attributes()
            .iter()
            .map(|a| self.render_attr(a))
            .collect::<Vec<_>>()
            .join(" ");

        format!("<merror {attrs}>{content}</merror>")
    }

    fn render_frac(&self, frac: &crate::elements::Frac) -> Self::Output {
        let num = self.render_elements(frac.num());
        let denom = self.render_elements(frac.denom());

        let attributes = frac
            .attributes()
            .iter()
            .map(|a| match a {
                FracAttr::Global(ga) => self.render_attr(ga),
                FracAttr::LineThickness(lt) => format!(r#"linethickness="{lt}""#),
            })
            .collect::<Vec<_>>()
            .join(" ");

        format!("<mfrac {attributes}>{num}{denom}</mfrac>")
    }

    fn render_ident(&self, ident: &crate::elements::Ident) -> Self::Output {
        let content = ident.ident();
        let attrs = ident
            .attributes()
            .iter()
            .map(|a| self.render_attr(a))
            .collect::<Vec<_>>()
            .join(" ");

        format!("<mi {attrs}>{content}</mi>")
    }

    fn render_multiscripts(
        &self,
        multiscripts: &crate::elements::scripted::Multiscripts,
    ) -> Self::Output {
        let content = self.render_elements(multiscripts.content());
        let attr = multiscripts
            .attributes()
            .iter()
            .map(|a| self.render_attr(a))
            .collect::<Vec<_>>()
            .join(" ");

        format!("<mmultiscripts {attr}>{content}</mmultiscripts>")
    }

    fn render_prescripts(&self, prescripts: &Prescripts) -> Self::Output {
        let attr = prescripts
            .attributes()
            .iter()
            .map(|a| self.render_attr(a))
            .collect::<Vec<_>>()
            .join(" ");
        format!("<mprescripts {attr}/>")
    }

    fn render_num(&self, num: &crate::elements::Num) -> Self::Output {
        let num_str = num.num();
        let attr = num
            .attributes()
            .iter()
            .map(|a| self.render_attr(a))
            .collect::<Vec<_>>()
            .join(" ");

        format!("<mn {attr}>{num_str}</mn>")
    }

    fn render_operator(&self, operator: &crate::elements::Operator) -> Self::Output {
        let op = operator.op();
        let attr = operator
            .attributes()
            .iter()
            .map(|a| match a {
                OperatorAttr::Global(ga) => self.render_attr(ga),
                OperatorAttr::Form(form) => format!(r#"form="{form}""#),
                OperatorAttr::Fence => String::from("fence=\"true\""),
                OperatorAttr::Separator => String::from("separator=\"true\""),
                OperatorAttr::LeftSpace(sp) => format!(r#"lspace="{sp}""#),
                OperatorAttr::RightSpace(sp) => format!(r#"rspace="{sp}""#),
                OperatorAttr::MaxSize(s) => format!(r#"maxsize="{s}""#),
                OperatorAttr::MinSize(s) => format!(r#"minsize="{s}""#),
                OperatorAttr::Stretchy => String::from("stretchy=\"true\""),
                OperatorAttr::Symmetric => String::from("symmetric=\"true\""),
                OperatorAttr::LargeOp => String::from("largeop=\"true\""),
                OperatorAttr::MovableLimits => String::from("movablelimits=\"true\""),
            })
            .collect::<Vec<_>>()
            .join(" ");

        format!("<mo {attr}>{op}</mo>")
    }

    fn render_padded(&self, padded: &crate::elements::Padded) -> Self::Output {
        let content = self.render_elements(padded.children());
        let attr = padded
            .attributes()
            .iter()
            .map(|a| match a {
                PaddedAttr::Width(w) => format!(r#"width="{w}""#),
                PaddedAttr::Height(h) => format!(r#"height="{h}""#),
                PaddedAttr::Depth(d) => format!(r#"depth="{d}""#),
                PaddedAttr::LeftSpace(ls) => format!(r#"lspace="{ls}""#),
                PaddedAttr::VerticalOffset(voffs) => format!(r#"voffset="{voffs}""#),
                PaddedAttr::Global(ga) => self.render_attr(ga),
            })
            .collect::<Vec<_>>()
            .join(" ");

        format!(r#"<mpadded {attr}>{content}</mpadded>"#)
    }

    fn render_phantom(&self, phantom: &crate::elements::grouping::Phantom) -> Self::Output {
        let content = self.render_elements(phantom.children());
        let attr = phantom
            .attributes()
            .iter()
            .map(|a| self.render_attr(a))
            .collect::<Vec<_>>()
            .join(" ");

        format!("<mphantom {attr}>{content}</mphantom>")
    }

    fn render_radical(&self, radical: &crate::elements::radicals::Radical) -> Self::Output {
        let content = self.render_elements(radical.content());
        let attr = radical
            .attributes()
            .iter()
            .map(|a| self.render_attr(a))
            .collect::<Vec<_>>()
            .join(" ");

        if radical.is_square() {
            format!(r#"<msqrt {attr}>{content}</msqrt>"#)
        } else {
            let index = self.render_num(&Num::from(radical.index()));

            format!(r#"<mroot {attr}>{content}{index}</mroot>"#)
        }
    }

    fn render_row(&self, row: &crate::elements::grouping::Row) -> Self::Output {
        let content = self.render_elements(row.children());
        let attr = row
            .attributes()
            .iter()
            .map(|a| self.render_attr(a))
            .collect::<Vec<_>>()
            .join(" ");

        format!("<mrow {attr}>{content}</mrow>")
    }

    fn render_semantics(&self, semantics: &crate::elements::Semantics) -> Self::Output {
        let content = self.render_elements(semantics.children());
        let attrs = semantics
            .attributes()
            .iter()
            .map(|a| self.render_attr(a))
            .collect::<Vec<_>>()
            .join(" ");

        format!("<semantics {attrs}>{content}</semantics>")
    }

    fn render_space(&self, space: &crate::elements::Space) -> Self::Output {
        let attrs = space
            .attributes()
            .iter()
            .map(|a| match a {
                SpaceAttr::Width(w) => format!(r#"width="{w}""#),
                SpaceAttr::Height(h) => format!(r#"height="{h}""#),
                SpaceAttr::Depth(d) => format!(r#"depth="{d}""#),
                SpaceAttr::Global(ga) => self.render_attr(ga),
            })
            .collect::<Vec<_>>()
            .join(" ");

        format!("<mspace {attrs}/>")
    }

    fn render_str_literal(&self, str_literal: &crate::elements::StrLiteral) -> Self::Output {
        let content = str_literal.content();
        let attr = str_literal
            .attributes()
            .iter()
            .map(|a| self.render_attr(a))
            .collect::<Vec<_>>()
            .join(" ");

        format!("<ms {attr}>{content}</ms>")
    }

    fn render_style(&self, style: &crate::elements::grouping::Style) -> Self::Output {
        let content = self.render_elements(style.children());
        let attr = style
            .attributes()
            .iter()
            .map(|a| self.render_attr(a))
            .collect::<Vec<_>>()
            .join(" ");

        format!("<mstyle {attr}>{content}</mstyle>")
    }

    fn render_subsup(&self, sub_sup: &crate::elements::scripted::SubSup) -> Self::Output {
        let sub = sub_sup.sub();
        let sup = sub_sup.sup();

        let (tag, formatting) = match (sub, sup) {
            (None, None) => unreachable!("SubSup element must have at least one of sub or sup."),
            (None, Some(sup)) => ("msup", self.render_elements(sup)),
            (Some(sub), None) => ("msub", self.render_elements(sub)),
            (Some(sub), Some(sup)) => {
                let tag = "msubsup";

                let mut formatting = self.render_elements(sub);
                formatting.push_str(&self.render_elements(sup));

                (tag, formatting)
            }
        };

        let base = self.render_elements(sub_sup.base());

        let attr = sub_sup
            .attributes()
            .iter()
            .map(|a| self.render_attr(a))
            .collect::<Vec<_>>()
            .join(" ");

        format!("<{tag} {attr}>{base}{formatting}</{tag}>")
    }

    fn render_table(&self, table: &crate::elements::Table) -> Self::Output {
        let mut rows = String::default();
        let mut cells = String::default();

        for row in table.rows() {
            for cell in row.cells() {
                let cell_content = self.render_elements(cell.children());
                let cell_attr = cell
                    .attributes()
                    .iter()
                    .map(|a| match a {
                        TableCellAttr::ColumnSpan(cs) => format!(r#"columnspan="{cs}""#),
                        TableCellAttr::RowSpan(rs) => format!(r#"rowspan="{rs}""#),
                        TableCellAttr::Global(ga) => self.render_attr(ga),
                    })
                    .collect::<Vec<_>>()
                    .join(" ");

                cells.push_str("<mtd ");
                cells.push_str(&cell_attr);
                cells.push('>');
                cells.push_str(&cell_content);
                cells.push_str("</mtd>");
            }

            let row_attr = row
                .attributes()
                .iter()
                .map(|a| self.render_attr(a))
                .collect::<Vec<_>>()
                .join(" ");

            rows.push_str("<mtr ");
            rows.push_str(&row_attr);
            rows.push('>');
            rows.push_str(&cells);
            rows.push_str("</mtr>");

            cells.clear();
        }

        let table_attr = table
            .attributes()
            .iter()
            .map(|a| self.render_attr(a))
            .collect::<Vec<_>>()
            .join(" ");

        format!(r#"<mtable {table_attr}>{rows}</mtable>"#)
    }

    fn render_text(&self, text: &crate::elements::Text) -> Self::Output {
        let content = text.text();
        let attr = text
            .attributes()
            .iter()
            .map(|a| self.render_attr(a))
            .collect::<Vec<_>>()
            .join(" ");

        format!("<mtext {attr}>{content}</mtext>")
    }

    fn render_underover(&self, under_over: &crate::elements::scripted::UnderOver) -> Self::Output {
        let under = under_over.under();
        let over = under_over.over();

        let (tag, formatting) = match (under, over) {
            (None, None) => unreachable!("SubSup element must have at least one of sub or sup."),
            (None, Some(over)) => ("munder", self.render_elements(over)),
            (Some(under), None) => ("mover", self.render_elements(under)),
            (Some(under), Some(over)) => {
                let tag = "munderover";

                let mut formatting = self.render_elements(under);
                formatting.push_str(&self.render_elements(over));

                (tag, formatting)
            }
        };

        let base = self.render_elements(under_over.expr());

        let attr = under_over
            .attributes()
            .iter()
            .map(|a| match a {
                UnderOverAttr::AccentUnder => String::from(r#"accentunder="true""#),
                UnderOverAttr::AccentOver => String::from(r#"accent="true""#),
                UnderOverAttr::Global(ga) => self.render_attr(ga),
            })
            .collect::<Vec<_>>()
            .join(" ");

        format!("<{tag} {attr}>{base}{formatting}</{tag}>")
    }

    fn render_attr(&self, attr: &crate::attributes::Attribute) -> Self::Output {
        match attr {
            Attribute::Class(c) => format!("class=\"{}\"", c),
            Attribute::Data { name, value } => format!(r#"data-{name}="{value}""#),
            Attribute::Dir(dir) => match dir {
                Dir::RightToLeft => String::from(r#"dir="rtl""#),
                Dir::LeftToRight => String::from(r#"dir="ltr""#),
            },
            Attribute::DisplayStyle(d) => {
                if *d {
                    String::from(r#"display="normal"#)
                } else {
                    String::from(r#"display="compact""#)
                }
            }
            Attribute::Id(id) => format!(r#"id="{id}""#),
            Attribute::MathBackground(c) => format!(r#"mathbackground="{c}""#),
            Attribute::MathColor(c) => format!(r#"mathcolor="{c}""#),
            Attribute::MathSize(s) => format!(r#"mathsize="{s}""#),
            Attribute::Nonce(n) => format!(r#"nonce="{n}""#),
            Attribute::ScriptLevel(sl) => match sl {
                ScriptLevel::Add(num) => format!(r#"scriptlevel="+{}""#, num),
                ScriptLevel::Sub(num) => format!(r#"scriptlevel="-{}""#, num),
                ScriptLevel::Num(num) => format!(r#"scriptlevel="{}""#, num),
            },
            Attribute::Style(st) => format!(r#"style="{st}""#),
            Attribute::TabIndex(ti) => format!(r#"tabindex="{ti}""#),
            Attribute::OnHandler { name, handler } => format!(r#"on{name}="{handler}""#),
            Attribute::MathVariant(mv) => format!(r#"mathvariant="{mv}""#),
        }
    }

    fn render_mathml(&self, mathml: &crate::MathMl) -> Self::Output {
        let content = self.render_elements(&mathml.content);

        let attr = mathml
            .attr
            .iter()
            .map(|a| match a {
                MathMlAttr::Display(d) => format!(r#"display="{d}""#),
                MathMlAttr::AltText(alt_t) => format!(r#"alttext="{alt_t}""#),
                MathMlAttr::Global(a) => self.render_attr(a),
            })
            .collect::<Vec<_>>()
            .join(" ");

        format!("<math {attr}>{content}</math>")
    }
}
