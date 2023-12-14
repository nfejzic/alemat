use alemat::{
    elements::{radicals::Radical, scripted::SubSup, Ident, Num, Operator},
    MathMl,
};

#[test]
fn sqrt() {
    let output = MathMl::with_content(
        Radical::builder()
            .index(Num::from(2))
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
    .render();

    crate::snap_test!(output, name: "radicals_sqrt");
}

#[test]
fn root() {
    let output = MathMl::with_content(
        Radical::builder()
            .index(Num::from(3))
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
    .render();

    crate::snap_test!(output, name: "radicals_root");
}

#[test]
fn root_expr() {
    let output = MathMl::with_content(
        Radical::builder()
            .index(alemat::children![
                Ident::from("n"),
                Operator::plus(),
                Num::from(1)
            ])
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
    .render();

    crate::snap_test!(output, name: "radicals_root_expr");
}
