use alemat::{
    attributes::Attribute,
    elements::{
        grouping::{Action, ActionAttr, Error, Phantom, Prescripts, Row, Style},
        scripted::{Multiscripts, SubSup},
        Frac, Ident, Num, OpForm, Operator,
    },
    row, MathMl, MathMlFormatter,
};

use crate::snap_test;

#[test]
fn action() {
    let output = MathMl::with_content(
        Action::builder()
            .content(Ident::from("x"))
            .attr([ActionAttr::Global(Attribute::Id(String::from("action-id")))])
            .build(),
    )
    .render_with(&mut MathMlFormatter);

    snap_test!(output, name: "grouping_action");
}

#[test]
fn merror() {
    let output = MathMl::with_content(
        Error::builder()
            .content(alemat::row![
                Ident::from("x"),
                Operator::from("+"),
                Ident::from("y"),
            ])
            .build(),
    )
    .render_with(&mut MathMlFormatter);

    snap_test!(output, name: "grouping_merror");
}

#[test]
fn mmultiscripts() {
    let output = MathMl::with_content(
        Multiscripts::builder()
            .content(alemat::children![
                Num::from(1),
                Num::from(2),
                Num::from(3),
                Row::default(),
                Num::from(5),
                Prescripts::default(),
                Num::from(6),
                Row::default(),
                Num::from(8),
                Num::from(9),
            ])
            .build(),
    )
    .render_with(&mut MathMlFormatter);
    snap_test!(output, name: "grouping_mmultiscripts");
}

#[test]
fn mphantom() {
    let output = MathMl::with_content(
        Frac::builder()
            .num(alemat::row![
                Ident::from("x"),
                Operator::from("+"),
                Ident::from("y"),
                Operator::from("+"),
                Ident::from("z"),
            ])
            .denom(alemat::row![
                Ident::from("x"),
                Phantom::from(alemat::children![
                    Operator::builder().op("+").attr([OpForm::Infix]).build(),
                    Ident::from("y")
                ]),
                Operator::from("+"),
                Ident::from("z")
            ])
            .build(),
    )
    .render_with(&mut MathMlFormatter);

    snap_test!(output, name: "grouping_mphantom");
}

#[test]
fn mrow() {
    let output = MathMl::with_content(row![Ident::from("x"), Operator::from("+"), Num::from(42)])
        .render_with(&mut MathMlFormatter);

    snap_test!(output, name: "grouping_row");
}

#[test]
fn mstyle() {
    let output = MathMl::with_content(
        Style::from(alemat::children![SubSup::builder()
            .base(Ident::from("x"))
            .subscript(Ident::from("i"))
            .build()])
        .with_attr([Attribute::MathColor(String::from("red"))]),
    )
    .render_with(&mut MathMlFormatter);

    snap_test!(output, name: "grouping_mstyle");
}
