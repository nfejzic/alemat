use alemat::{
    elements::{scripted::SubSup, Ident, Num, Operator},
    MathMl, MathMlAttr, MathMlFormatter,
};

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
fn integral() {
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
fn summation() {
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
