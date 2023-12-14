use crate::{attributes::Attribute, Element, Elements};

/// One of the values for `columnlines` attribute.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColumnLine {
    /// No line is drawn.
    None,

    /// A solid line is drawn.
    Solid,

    /// a dashed line is drawn.
    Dashed,
}

impl AsRef<str> for ColumnLine {
    fn as_ref(&self) -> &str {
        match self {
            ColumnLine::None => "none",
            ColumnLine::Solid => "solid",
            ColumnLine::Dashed => "dashed",
        }
    }
}

/// The `mtable` accepts the global [`Attribute`]s as well as `columnlines` that can be used to
/// render augmented matrix.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Table {
    rows: Vec<TableRow>,
    /// The `mtable` accepts the global [`Attribute`]s.
    attributes: Vec<TableAttr>,
}

impl Table {
    /// Get a reference to the [`Table`]s rows.
    pub fn rows(&self) -> &[TableRow] {
        &self.rows
    }

    /// Add a [`TableRow`] to the [`Table`].
    pub fn add_row(&mut self, row: TableRow) {
        self.rows.push(row);
    }

    /// Add multiple [`TableRow`]s to the [`Table`].
    pub fn add_rows<I, R>(&mut self, rows: I)
    where
        I: IntoIterator<Item = R>,
        R: Into<TableRow>,
    {
        self.rows.extend(rows.into_iter().map(Into::into));
    }

    /// Create a new instance of [`Table`] extended with the given [`TableRow`].
    pub fn with_row(mut self, row: TableRow) -> Self {
        self.rows.push(row);
        self
    }

    /// Create a new instance of [`Table`] extended with given [`TableRow`]s.
    pub fn with_rows<I, R>(mut self, rows: I) -> Self
    where
        I: IntoIterator<Item = R>,
        R: Into<TableRow>,
    {
        self.rows.extend(rows.into_iter().map(Into::into));
        self
    }

    /// Get a reference to the attributes of the [`Table`] element.
    pub fn attributes(&self) -> &[TableAttr] {
        &self.attributes
    }

    /// Add attributes.
    pub fn add_attr<I, A>(&mut self, attr: I)
    where
        I: IntoIterator<Item = A>,
        A: Into<TableAttr>,
    {
        self.attributes.extend(attr.into_iter().map(Into::into));
    }

    /// Create a new instance of [`Table`] with additional attributes.
    pub fn with_attr<I, A>(mut self, attr: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<TableAttr>,
    {
        self.attributes.extend(attr.into_iter().map(Into::into));
        self
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
    ($([$($cell:expr),* $(,)?]),* $(,)?) => {
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
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    /// Add attributes.
    pub fn add_attr<I, A>(&mut self, attr: I)
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        self.attr.extend(attr.into_iter().map(Into::into));
    }

    /// Add a [`TableCell`] to this instance of [`TableRow`].
    pub fn add_cell(&mut self, cell: TableCell) {
        self.cells.push(cell);
    }

    /// Add multiple [`TableCell`]s to this instance of [`TableRow`].
    pub fn add_cells<I, C>(&mut self, cells: I)
    where
        I: IntoIterator<Item = C>,
        C: Into<TableCell>,
    {
        self.cells.extend(cells.into_iter().map(Into::into));
    }

    /// Create a new instance of [`TableRow`] extended with the given [`TableCell`].
    pub fn with_cell(mut self, cell: TableCell) -> Self {
        self.cells.push(cell);
        self
    }

    /// Create a new instance of [`TableRow`] extended with given [`TableCell`]s.
    pub fn with_cells<I, C>(mut self, cells: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Into<TableCell>,
    {
        self.cells.extend(cells.into_iter().map(Into::into));
        self
    }

    /// Get a reference to the cells of the [`TableRow`] element.
    pub fn cells(&self) -> &[TableCell] {
        &self.cells
    }

    /// Get a reference to all attributes of the [`TableRow`] element.
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
        value.into_iter().collect()
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

use super::IntoElements;

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
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TableCell {
    children: Elements,
    attr: Vec<TableCellAttr>,
}

impl<T> From<T> for TableCell
where
    T: Into<Element>,
{
    fn from(value: T) -> Self {
        Self {
            children: Elements(vec![value.into()]),
            attr: Vec::default(),
        }
    }
}

impl From<Elements> for TableCell {
    fn from(children: Elements) -> Self {
        Self {
            children,
            attr: Vec::default(),
        }
    }
}

impl<const N: usize, I: Into<Element>> From<[I; N]> for TableCell {
    fn from(value: [I; N]) -> Self {
        Self {
            children: value.into_elements(),
            attr: Default::default(),
        }
    }
}

impl TableCell {
    /// Get a reference to the children of the [`TableCell`] element.
    pub fn children(&self) -> &[Element] {
        &self.children
    }

    /// Get a reference to all attributes of the [`TableCell`] element.
    pub fn attributes(&self) -> &[TableCellAttr] {
        &self.attr
    }

    /// Create a [`TableCell`] with the given content.
    pub fn with_content(content: impl IntoElements) -> Self {
        Self {
            children: content.into_elements(),
            attr: Default::default(),
        }
    }

    /// Add attributes.
    pub fn add_attr<I, A>(&mut self, attr: I)
    where
        I: IntoIterator<Item = A>,
        A: Into<TableCellAttr>,
    {
        self.attr.extend(attr.into_iter().map(Into::into));
    }

    /// Create a new instance of [`TableCell`] with additional attributes.
    pub fn with_attr<I, A>(mut self, attr: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<TableCellAttr>,
    {
        self.attr.extend(attr.into_iter().map(Into::into));
        self
    }
}
