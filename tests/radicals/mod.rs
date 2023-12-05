use alemat::{
    elements::{radicals::Radical, scripted::SubSup, Ident, Num, Operator},
    MathMl, MathMlFormatter,
};

#[test]
fn sqrt() {
    let output = MathMl::with_content(
        Radical::builder()
            .index("2")
            .content(alemat::children![
                Num::from(1),
                Operator::from("+"),
                SubSup::builder()
                    .base(Ident::from("n"))
                    .subscript(Num::from(2))
                    .supscript(Num::from(3))
                    .build(),
            ])
            .build(),
    )
    .render_with(&mut MathMlFormatter);

    crate::snap_test!(output, name: "radicals_sqrt");
}

#[test]
fn root() {
    let output = MathMl::with_content(
        Radical::builder()
            .index("3")
            .content(alemat::children![
                Num::from(1),
                Operator::from("+"),
                SubSup::builder()
                    .base(Ident::from("n"))
                    .subscript(Num::from(2))
                    .supscript(Num::from(3))
                    .build(),
            ])
            .build(),
    )
    .render_with(&mut MathMlFormatter);

    crate::snap_test!(output, name: "radicals_root");
}
