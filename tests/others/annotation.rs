use alemat::{
    children,
    elements::{Annotation, Ident, Operator},
    MathMl,
};

#[test]
fn annotation() {
    let out = MathMl::with_content(
        Annotation::builder()
            .content(String::from("This is an annotation"))
            .build(),
    )
    .render();

    crate::snap_test!(out, name: "others_annotation");
}

#[test]
fn annotation_xml() {
    let out = MathMl::with_content(
        Annotation::builder()
            .content(children![
                Ident::from("x"),
                Operator::plus(),
                Ident::from("y"),
            ])
            .build(),
    )
    .render();

    crate::snap_test!(out, name: "others_annotation_xml");
}
