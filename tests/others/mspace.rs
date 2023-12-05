use alemat::{
    attributes::Attribute,
    elements::{Frac, Num, Space, SpaceAttr},
    MathMl,
};

#[test]
fn mspace() {
    let out = MathMl::with_content(alemat::children![
        Num::from(1),
        Space::default().with_attr([
            SpaceAttr::Width("1em".into()),
            SpaceAttr::Global(Attribute::Style("border-top: 1px solid blue".into()))
        ]),
        Frac::builder()
            .num(alemat::row![
                Num::from(2),
                Space::default().with_attr([
                    SpaceAttr::Depth("1em".into()),
                    SpaceAttr::Global(Attribute::Style("border-left: 1px solid blue".into()))
                ]),
            ])
            .denom(alemat::row![
                Num::from(3),
                Space::default().with_attr([
                    SpaceAttr::Height("2em".into()),
                    SpaceAttr::Global(Attribute::Style("border-left: 1px solid blue".into()))
                ]),
            ])
            .build()
    ])
    .render();

    crate::snap_test!(out, name: "others_mspace");
}
