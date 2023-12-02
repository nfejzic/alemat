use alemat::{elements::Text, MathMl, MathMlFormatter};

#[test]
fn text() {
    let out = MathMl::with_content(Text::from("Hello world")).render(&mut MathMlFormatter);

    crate::snap_test!(out, name: "others_text");
}