use alemat::{elements::Num, MathMl};

#[test]
fn number() {
    let out = MathMl::with_content(Num::from(1)).render();

    crate::snap_test!(out, name: "others_number");
}
