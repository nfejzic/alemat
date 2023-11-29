use alemat::{
    attributes::Attribute,
    elements::{radicals::Radical, Annotation, Ident, Num, Table},
    row,
};
use alemat::{table, table_row};

#[test]
fn test_annotation_api() {
    let _annotation = Annotation::builder()
        .content(Num::from(42))
        .attr([Attribute::Id("Some id".into())])
        .build();
}

#[test]
fn test_row_api() {
    let _row = row![
        Num::from(42),
        Ident::from("Hello"),
        Radical::builder()
            .index("3")
            .content(Ident::from("x"))
            .build()
    ];
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
        table_row![Ident::from("x"), Num::from(41)],
        table_row![Num::from(42), Num::from(43)],
        table_row![Num::from(44), Num::from(45)],
    ]);

    let _table = table![
        [Ident::from("x"), Num::from(41)],
        [Ident::from("x"), Num::from(41)]
    ];
}
