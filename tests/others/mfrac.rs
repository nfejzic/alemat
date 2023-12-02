use alemat::{
    elements::{Frac, Ident},
    MathMl, MathMlFormatter,
};

#[test]
fn frac() {
    let out = MathMl::with_content(
        Frac::builder()
            .num(Ident::from("x"))
            .denom(Ident::from("y"))
            .build(),
    )
    .render(&mut MathMlFormatter);

    crate::snap_test!(out, name: "others_frac");
}
