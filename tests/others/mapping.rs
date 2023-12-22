use alemat::{
    elements::{grouping::Row, Frac, Ident},
    MathMl,
};

#[test]
fn map_in_row() {
    let out = MathMl::with_content(
        Frac::builder()
            .num(Ident::from("x"))
            .denom(Ident::from("y"))
            .build(),
    )
    .map(Row::from)
    .render();

    crate::snap_test!(out, name: "others_map_in_row");
}
