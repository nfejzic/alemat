use alemat::{
    elements::{ColumnLine, Frac, Ident, Num, Operator, TableAttr},
    MathMl, MathMlFormatter,
};

#[test]
fn matrix() {
    let out = MathMl::with_content(alemat::children![
        Frac::builder()
            .num(Ident::from("A"))
            .denom(Num::from(2))
            .build(),
        Operator::eq(),
        alemat::row![
            Operator::lparens(),
            alemat::table![
                [Num::from(1), Num::from(2), Num::from(3)],
                [Num::from(4), Num::from(5), Num::from(6)],
                [Num::from(7), Num::from(8), Num::from(9)],
            ],
            Operator::rparens(),
        ]
    ])
    .render(&mut MathMlFormatter);

    crate::snap_test!(out, name: "others_table_matrix");
}

#[test]
fn matrix_augmented() {
    let out = MathMl::with_content(alemat::children![
        Frac::builder()
            .num(Ident::from("A"))
            .denom(Num::from(2))
            .build(),
        Operator::eq(),
        alemat::row![
            Operator::lparens(),
            alemat::table![
                [Num::from(1), Num::from(2), Num::from(3)],
                [Num::from(4), Num::from(5), Num::from(6)],
                [Num::from(7), Num::from(8), Num::from(9)],
            ]
            .with_attr([TableAttr::ColumnLines(vec![
                ColumnLine::None,
                ColumnLine::Solid,
                ColumnLine::None
            ])]),
            Operator::rparens(),
        ]
    ])
    .render(&mut MathMlFormatter);

    crate::snap_test!(out, name: "others_table_matrix_augmented");
}

#[test]
fn matrix_square() {
    let out = MathMl::with_content(alemat::children![
        Frac::builder()
            .num(Ident::from("A"))
            .denom(Num::from(2))
            .build(),
        Operator::eq(),
        alemat::row![
            Operator::lbracket(),
            alemat::table![
                [Num::from(1), Num::from(2), Num::from(3)],
                [Num::from(4), Num::from(5), Num::from(6)],
                [Num::from(7), Num::from(8), Num::from(9)],
            ],
            Operator::rbracket(),
        ]
    ])
    .render(&mut MathMlFormatter);

    crate::snap_test!(out, name: "others_table_matrix_square");
}
