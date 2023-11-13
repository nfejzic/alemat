use crate::{attributes::Attribute, MathMl};

pub type Matrix = Table;

pub enum ColumnLine {
    /// No line is drawn.
    None,

    /// A solid line is drawn.
    Solid,

    /// a dashed line is drawn.
    Dashed,
}

/// The `mtable` accepts the global [`Attribute`]s as well as `columnlines` that can be used to
/// render augmented matrix.
pub enum TableAttr {
    /// The `columnlines` attribute is a space-separated list of values, one for each column.
    ColumnLines(Vec<ColumnLine>),

    /// One of the global [`Attribute`]s.
    Global(Attribute),
}

/// The `mtable` is laid out as an inline-table and sets displaystyle to false. The user agent
/// stylesheet must contain the following rules in order to implement these properties:
///
/// The `mtable` accepts the global [`Attribute`]s.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Table {
    rows: Vec<TableRow>,
    /// The `mtable` accepts the global [`Attribute`]s.
    attributes: Vec<Attribute>,
}

/// The `mtr` is laid out as `table-row`. The user agent stylesheet must contain the following
/// rules in order to implement that behavior:
///
/// ```css
/// mtr {
///   display: table-row;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TableRow {
    /// Table cells (`mtd`) of this table row.
    cells: Vec<TableCell>,
    /// The `mtr` accepts the global [`Attribute`]s.
    attr: Vec<Attribute>,
}

/// The `mtd` accepts the global [`Attribute`]s as well as `columnspan` and `rowspan`.
///
/// The `columnspan` (respectively `rowspan`) attribute has the same syntax and semantics as the
/// colspan (respectively rowspan) attribute on the `<td>` element from HTML. In particular, the
/// parsing of these attributes is handled as described in the algorithm for processing rows,
/// always reading `colspan` as `columnspan`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TableCellAttr {
    /// Has the same syntax and semantics as the `colspan` attribute on the `<td>` element from
    /// HTML.
    ColumnSpan(String),

    /// Has the same syntax and semantics as the `rowspan` attribute on the `<td>` element from
    /// HTML.
    RowSpan(String),

    /// One of the global [`Attribute`]s.
    Global(Attribute),
}

/// The `mtd` is laid out as a `table-cell` with content centered in the cell and a default
/// padding. The user agent stylesheet must contain the following rules:
///
/// ```css
/// mtd {
///   display: table-cell;
///   /* Centering inside table cells should rely on box alignment properties.
///      See https://github.com/w3c/mathml-core/issues/156 */
///   text-align: center;
///   padding: 0.5ex 0.4em;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TableCell {
    children: MathMl,
    attr: Vec<TableCellAttr>,
}
