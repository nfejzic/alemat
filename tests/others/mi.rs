use alemat::{elements::Ident, MathMl};

#[test]
fn ident() {
    let out = MathMl::with_content(Ident::big_psi()).render();

    crate::snap_test!(out, name: "others_ident");
}
