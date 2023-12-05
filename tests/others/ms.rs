use alemat::{elements::StrLiteral, MathMl, MathMlFormatter};

#[test]
fn str_lit() {
    let out =
        MathMl::with_content(StrLiteral::from("Hello there")).render_with(&mut MathMlFormatter);

    crate::snap_test!(out, name: "others_str_literal");
}
