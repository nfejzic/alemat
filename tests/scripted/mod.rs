use alemat::elements::scripted::{SubSup, UnderOver};
use alemat::elements::{Ident, Num, Operator};
use alemat::{MathMl, MathMlAttr, MathMlFormatter};

#[test]
fn subsup() {
    let out = MathMl::with_content(
        SubSup::builder()
            .subscript(Ident::from("0"))
            .supscript(Ident::from("1"))
            .base(Ident::from("x"))
            .build(),
    )
    .render(&mut MathMlFormatter);

    crate::snap_test!(out, name: "subsup_subsup");
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
    .with_attr([MathMlAttr::Display(String::from("block"))])
    .render(&mut MathMlFormatter);

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
    .with_attr([MathMlAttr::Display(String::from("block"))])
    .render(&mut MathMlFormatter);

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
    .render(&mut MathMlFormatter);

    crate::snap_test!(out, name: "underover_underover");
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
    .with_attr([MathMlAttr::Display(String::from("block"))])
    .render(&mut MathMlFormatter);

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
    .with_attr([MathMlAttr::Display(String::from("block"))])
    .render(&mut MathMlFormatter);

    crate::snap_test!(out, name: "underover_summation");
}
