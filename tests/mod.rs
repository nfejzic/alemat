// use alemat::{
//     attributes::Attribute,
//     children,
//     elements::{radicals::Radical, Annotation, Ident, Num, Operator, Table, TableCell, TableRow},
//     row, MathMl, MathMlFormatter,
// };
// use alemat::{table, table_row};

mod grouping;

// #[test]
// fn test_annotation_api() {
//     let _annotation = Annotation::builder()
//         .content(Num::from(42))
//         .attr([Attribute::Id("Some id".into())])
//         .build();
//
//     let math = MathMl::with_content(_annotation);
//
//     let output = math.render(&mut MathMlFormatter);
//
//     assert_eq!(
//         output,
//         r#"<math><annotation-xml id="Some id"><mn >42</mn></annotation-xml></math>"#
//     );
// }
//
// #[test]
// fn test_row_api() {
//     let _row = row![
//         Num::from(42),
//         Ident::from("Hello"),
//         Radical::builder()
//             .index("3")
//             .content(Ident::from("x"))
//             .build()
//     ];
// }
//
// #[test]
// fn test_general_api() {
//     let r1 = Radical::builder()
//         .content(row![Num::from(42), Operator::from("+"), Ident::from("c")])
//         .index("2")
//         .build();
//
//     let output = MathMl::with_content(r1).render(&mut MathMlFormatter);
//     assert_eq!(
//         output,
//         r#"<math><msqrt ><mrow ><mn >42</mn><mo >+</mo><mi >c</mi></mrow></msqrt></math>"#
//     );
//
//     let r2 = Radical::builder()
//         .content(children![
//             Num::from(42),
//             Operator::from("+"),
//             Ident::from("c")
//         ])
//         .index("2")
//         .build();
//
//     let output = MathMl::with_content(r2).render(&mut MathMlFormatter);
//     assert_eq!(
//         output,
//         r#"<math><msqrt ><mn >42</mn><mo >+</mo><mi >c</mi></msqrt></math>"#
//     );
// }
//
// #[test]
// fn test_table_api() {
//     let _table = Table::from([
//         // this should be a row
//         [Num::from(40), Num::from(41)],
//         [Num::from(42), Num::from(43)],
//         [Num::from(44), Num::from(45)],
//     ]);
//
//     let output = MathMl::with_content(_table).render(&mut MathMlFormatter);
//     let row1 = r"<mtr ><mtd ><mn >40</mn></mtd><mtd ><mn >41</mn></mtd></mtr>";
//     let row2 = r"<mtr ><mtd ><mn >42</mn></mtd><mtd ><mn >43</mn></mtd></mtr>";
//     let row3 = r"<mtr ><mtd ><mn >44</mn></mtd><mtd ><mn >45</mn></mtd></mtr>";
//     let table = format!(r#"<math><mtable >{row1}{row2}{row3}</mtable></math>"#);
//
//     assert_eq!(output, table);
//
//     let _ = TableRow::from([
//         TableCell::from(Num::from(42)),
//         TableCell::from(Ident::from("x")).with_attr([Attribute::Id(String::from("identifier"))]),
//     ]);
//
//     let _table = Table::from([
//         table_row![Ident::from("x"), Num::from(41)],
//         table_row![Num::from(42), Num::from(43)],
//         table_row![Num::from(44), Num::from(45)],
//     ]);
//
//     let _table = table![
//         [Ident::from("x"), Num::from(41)],
//         [Ident::from("x"), Num::from(41)]
//     ];
// }

// fn bla() {
//     let mut settings = insta::Settings::clone_current();
//
//     let base_dir = env!("CARGO_MANIFEST_DIR");
//
//     let path = Path::new(base_dir).join("tests/snapshots");
//     settings.set_snapshot_path(path)
//     settings.set_prepend_module_to_snapshot(false);
// }

macro_rules! snap_test {
    ($input:expr $(, name: $name:expr)?) => {
        let mut settings = insta::Settings::clone_current();

        let base_dir = env!("CARGO_MANIFEST_DIR");

        let path = std::path::Path::new(base_dir).join("tests/snapshots");
        settings.set_snapshot_path(dbg!(path));
        settings.set_prepend_module_to_snapshot(false);

        settings.bind(|| {
            insta::assert_snapshot!($($name,)? $input);
        });
    };
}

// use std::path::Path;

use snap_test;
