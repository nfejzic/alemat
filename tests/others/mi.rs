use alemat::{elements::Ident, MathMl, MathMlFormatter};

#[test]
fn ident() {
    let out = MathMl::with_content(Ident::big_psi()).render_with(&mut MathMlFormatter);

    crate::snap_test!(out, name: "others_ident");
}
