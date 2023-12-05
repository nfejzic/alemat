use alemat::{elements::Text, MathMl};

#[test]
fn text() {
    let out = MathMl::with_content(Text::from("Hello world")).render();

    crate::snap_test!(out, name: "others_text");
}
