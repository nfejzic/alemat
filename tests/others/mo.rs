use alemat::{elements::Operator, MathMl};

#[test]
fn operator_product() {
    let out = MathMl::with_content(Operator::prod()).render();

    crate::snap_test!(out, name: "operator_product");
}
