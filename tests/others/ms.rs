use alemat::{elements::StrLiteral, MathMl};

#[test]
fn str_lit() {
    let out = MathMl::with_content(StrLiteral::from("Hello there")).render();

    crate::snap_test!(out, name: "others_str_literal");
}
