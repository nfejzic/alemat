use alemat::elements::scripted::{SubSup, UnderOver};
use alemat::elements::{Ident, Num, Operator, Text};
use alemat::{DisplayAttr, MathMl, MathMlAttr};

#[test]
fn subsup() {
    let out = MathMl::with_content(
        SubSup::builder()
            .subscript(Ident::from("0"))
            .supscript(Ident::from("1"))
            .base(Ident::from("x"))
            .build(),
    )
    .render();

    crate::snap_test!(out, name: "subsup_subsup");
}

#[test]
fn subsup_many() {
    let sub = alemat::children![
        Operator::lparens(),
        Ident::from("n"),
        Operator::eq(),
        Num::from(0),
        Operator::rparens(),
    ];

    let sup = alemat::children![Ident::from("k"), Operator::plus(), Num::from(1)];

    let out = MathMl::with_content(alemat::children![SubSup::builder()
        .base(Operator::integral())
        .subscript(sub)
        .supscript(sup)
        .build()])
    .render();

    crate::snap_test!(out, name: "subsup_many");
}

#[test]
fn subsup_integral() {
    let out = MathMl::with_content(alemat::children![
        SubSup::builder()
            .base(Operator::integral())
            .subscript(Ident::from("0"))
            .supscript(Ident::from("1"))
            .build(),
        Ident::from("x"),
        Ident::from("dx"),
    ])
    .with_attr([MathMlAttr::Display(DisplayAttr::Block)])
    .render();

    crate::snap_test!(out, name: "subsup_integral");
}

#[test]
fn subsup_summation() {
    let out = MathMl::with_content(alemat::children![
        SubSup::builder()
            .base(Operator::sum())
            .subscript(alemat::row![Ident::from("i"), Operator::eq(), Num::from(0)])
            .supscript(Ident::from("k"))
            .build(),
        SubSup::builder()
            .base(Num::from(2))
            .supscript(Ident::from("i"))
            .build()
    ])
    .with_attr([MathMlAttr::Display(DisplayAttr::Block)])
    .render();

    crate::snap_test!(out, name: "subsup_summation");
}

#[test]
fn underover() {
    let out = MathMl::with_content(
        UnderOver::builder()
            .under(Ident::from("0"))
            .over(Ident::from("1"))
            .expr(Ident::from("x"))
            .build(),
    )
    .render();

    crate::snap_test!(out, name: "underover_underover");
}

#[test]
fn underover_many() {
    let under = alemat::children![
        Operator::lparens(),
        Ident::from("n"),
        Operator::eq(),
        Num::from(0),
        Operator::rparens(),
    ];

    let over = alemat::children![Ident::from("k"), Operator::plus(), Num::from(1)];

    let out = MathMl::with_content(alemat::children![UnderOver::builder()
        .expr(Operator::sum())
        .under(under)
        .over(over)
        .build()])
    .render();

    crate::snap_test!(out, name: "underover_many");
}

#[test]
fn underover_integral() {
    let out = MathMl::with_content(alemat::children![
        UnderOver::builder()
            .expr(Operator::integral())
            .under(Ident::from("0"))
            .over(Ident::from("1"))
            .build(),
        Ident::from("x"),
        Ident::from("dx"),
    ])
    .with_attr([MathMlAttr::Display(DisplayAttr::Block)])
    .render();

    crate::snap_test!(out, name: "underover_integral");
}

#[test]
fn underover_summation() {
    let out = MathMl::with_content(alemat::children![
        UnderOver::builder()
            .expr(Operator::sum())
            .under(alemat::row![Ident::from("i"), Operator::eq(), Num::from(0)])
            .over(Ident::from("k"))
            .build(),
        SubSup::builder()
            .base(Num::from(2))
            .supscript(Ident::from("i"))
            .build()
    ])
    .with_attr([MathMlAttr::Display(DisplayAttr::Block)])
    .render();

    crate::snap_test!(out, name: "underover_summation");
}

#[test]
fn under_brace() {
    let out = MathMl::with_content(alemat::children![UnderOver::builder()
        .expr(alemat::children![
            Num::from(1),
            Operator::plus(),
            Num::from(2)
        ])
        .under(
            UnderOver::builder()
                .expr(Operator::ubrace())
                .under(Text::from("Some expression"))
                .build()
        )
        .build()])
    .render();

    crate::snap_test!(out, name: "underover_under_brace");
}

#[test]
fn over_brace() {
    let out = MathMl::with_content(alemat::children![UnderOver::builder()
        .expr(alemat::children![
            Num::from(1),
            Operator::plus(),
            Num::from(2)
        ])
        .over(
            UnderOver::builder()
                .expr(Operator::obrace())
                .over(Text::from("Some expression"))
                .build()
        )
        .build()])
    .render();

    crate::snap_test!(out, name: "underover_over_brace");
}
