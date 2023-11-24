use alemat::{
    attributes::Attribute,
    elements::{Annotation, Ident, Num, Table},
};
use alemat::{row, table};

#[test]
fn test_annotation_api() {
    let _annotation = Annotation::builder()
        .content(Num::from(42))
        .attr([Attribute::Id("Some id".into())])
        .build();
}

#[test]
fn test_table_api() {
    let _table = Table::from([
        // this should be a row
        [Num::from(40), Num::from(41)],
        [Num::from(42), Num::from(43)],
        [Num::from(44), Num::from(45)],
    ]);

    let _table = Table::from([
        row![Ident::from("x"), Num::from(41)],
        row![Num::from(42), Num::from(43)],
        row![Num::from(44), Num::from(45)],
    ]);

    let _table = table![
        [Ident::from("x"), Num::from(41)],
        [Ident::from("x"), Num::from(41)]
    ];
}
