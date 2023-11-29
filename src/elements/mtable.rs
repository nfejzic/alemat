use crate::{attributes::Attribute, Element};

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

impl Table {
    pub fn rows(&self) -> &[TableRow] {
        &self.rows
    }

    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

crate::element_from_type!(Table => Table);

impl<R> FromIterator<R> for Table
where
    R: Into<TableRow>,
{
    fn from_iter<T: IntoIterator<Item = R>>(iter: T) -> Self {
        Self {
            rows: iter.into_iter().map(Into::into).collect(),
            attributes: Default::default(),
        }
    }
}

impl<I, C, const N: usize> From<I> for Table
where
    I: IntoIterator<Item = [C; N]>,
    C: Into<TableCell>,
{
    fn from(value: I) -> Self {
        Self::from_iter(value)
    }
}

impl Table {
    pub fn add_attr<I, A>(&mut self, attr: I)
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        self.attributes.extend(attr.into_iter().map(Into::into));
    }
}

/// Create a [`Table`] easily using this macro.
///
/// # Example
///
/// ```rust
/// use alemat::table;
/// use alemat::elements::{Ident, Num};
///
/// let table = table![
///     [Ident::from("x"), Num::from(41)],
///     [Ident::from("x"), Num::from(41)]
/// ];
/// ```
#[macro_export]
macro_rules! table {
    ($([$($cell:expr),*]),*) => {
        $crate::elements::Table::from([
            $(
            $crate::table_row![$($cell),*],
            )*
        ])
    }
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

impl<C> FromIterator<C> for TableRow
where
    C: Into<TableCell>,
{
    fn from_iter<T: IntoIterator<Item = C>>(iter: T) -> Self {
        Self {
            cells: iter.into_iter().map(Into::into).collect(),
            attr: Default::default(),
        }
    }
}

impl TableRow {
    pub fn add_attr<I, A>(&mut self, attr: I)
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        self.attr.extend(attr.into_iter().map(Into::into));
    }

    pub fn cells(&self) -> &[TableCell] {
        &self.cells
    }

    pub fn attributes(&self) -> &[Attribute] {
        &self.attr
    }
}

impl<I, C> From<I> for TableRow
where
    I: IntoIterator<Item = C>,
    C: Into<TableCell>,
{
    fn from(value: I) -> Self {
        Self {
            cells: value.into_iter().map(Into::into).collect(),
            attr: Default::default(),
        }
    }
}

/// Create a row of [`TableCell`]s. To be used in [`Table`].
///
/// # Example:
///
/// ```rust
/// use alemat::table_row;
/// use alemat::elements::{Ident, Num};
/// let row = table_row![Ident::from("x"), Num::from(42)];
///
/// // create a table
/// use alemat::elements::Table;
/// let table = Table::from([
///     table_row![Ident::from("x"), Num::from(42)],
///     table_row![Ident::from("y"), Num::from(43)],
/// ]);
/// ```
#[macro_export]
macro_rules! table_row {
    ($($cell:expr),* $(,)?) => {
         [$($crate::elements::TableCell::from($cell)),*]
    }
}

pub use table_row;

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

impl From<Attribute> for TableCellAttr {
    fn from(value: Attribute) -> Self {
        Self::Global(value)
    }
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
    children: Vec<Element>,
    attr: Vec<TableCellAttr>,
}

impl TableCell {
    pub fn children(&self) -> &[Element] {
        &self.children
    }

    pub fn attributes(&self) -> &[TableCellAttr] {
        &self.attr
    }
}

impl<T> From<T> for TableCell
where
    T: Into<Element>,
{
    fn from(value: T) -> Self {
        Self {
            children: vec![value.into()],
            attr: Default::default(),
        }
    }
}

impl<T> From<Vec<T>> for TableCell
where
    T: Into<Element>,
{
    fn from(value: Vec<T>) -> Self {
        Self {
            children: value.into_iter().map(Into::into).collect(),
            attr: Default::default(),
        }
    }
}

impl TableCell {
    pub fn add_attr<I, A>(&mut self, attr: I)
    where
        I: IntoIterator<Item = A>,
        A: Into<TableCellAttr>,
    {
        self.attr.extend(attr.into_iter().map(Into::into));
    }
}
