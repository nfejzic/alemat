use alemat::{elements::Num, MathMl, MathMlFormatter};

#[test]
fn number() {
    let out = MathMl::with_content(Num::from(1)).render(&mut MathMlFormatter);

    crate::snap_test!(out, name: "others_number");
}
