use alemat::{
    attributes::Attribute,
    elements::{Frac, Num, Padded, PaddedAttr},
    MathMl, MathMlFormatter,
};

#[test]
fn padded() {
    // example from https://www.w3.org/TR/mathml-core/#adjust-space-around-content-mpadded

    let out = MathMl::with_content(
        Padded::from(alemat::children![Frac::builder()
            .num(Num::from(23456))
            .denom(Num::from(78))
            .build()])
        .with_attr([
            PaddedAttr::LeftSpace("2em".into()),
            PaddedAttr::VerticalOffset("-1em".into()),
            PaddedAttr::Height("1em".into()),
            PaddedAttr::Depth("3em".into()),
            PaddedAttr::Width("7em".into()),
            PaddedAttr::Global(Attribute::Style("background: lightblue;".into())),
        ]),
    )
    .render_with(&mut MathMlFormatter);

    crate::snap_test!(out, name: "others_padded");
}
