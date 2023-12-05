use alemat::{elements::Operator, MathMl, MathMlFormatter};

#[test]
fn operator_product() {
    let out = MathMl::with_content(Operator::prod()).render_with(&mut MathMlFormatter);

    crate::snap_test!(out, name: "operator_product");
}
