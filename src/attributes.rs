//! Global attributes of MathML elements.

use std::fmt::Display;

/// Direction for [`Attribute::Dir`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dir {
    /// `rtl` direction.
    RightToLeft,
    /// `ltr` direction.
    LeftToRight,
}

/// ScriptLevel for [`Attribute::ScriptLevel`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScriptLevel {
    /// A positive number.
    Add(usize),
    /// A negative number.
    Sub(usize),
    /// A (positive) number.
    Num(usize),
}

/// MathVariant for [`Attribute::MathVariant`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MathVariant {
    /// Normal variant.
    Normal,
    /// Bold variant.
    Bold,
    /// Italic variant.
    Italic,
    /// Bold italic variant.
    BoldItalic,
    /// Double struck variant.
    DoubleStruck,
    /// Bold fraktur variant.
    BoldFraktur,
    /// Script variant.
    Script,
    /// Bold script variant.
    BoldScript,
    /// Fraktur variant.
    Fraktur,
    /// Sans serif variant.
    SansSerif,
    /// Bold sans serif variant.
    BoldSansSerif,
    /// Sans serif italic variant.
    SansSerifItalic,
    /// Sans serif bold italic variant.
    SansSerifBoldItalic,
    /// Monospace variant.
    Monospace,
    /// Initial variant.
    Initial,
    /// Tailed variant.
    Tailed,
    /// Looped variant.
    Looped,
    /// Stretched variant.
    Stretched,
}

impl Display for MathVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_str = match self {
            MathVariant::Normal => "normal",
            MathVariant::Bold => "bold",
            MathVariant::Italic => "italic",
            MathVariant::BoldItalic => "bold-italic",
            MathVariant::DoubleStruck => "double-struck",
            MathVariant::BoldFraktur => "bold-fraktur",
            MathVariant::Script => "script",
            MathVariant::BoldScript => "bold-script",
            MathVariant::Fraktur => "fraktur",
            MathVariant::SansSerif => "sans-serif",
            MathVariant::BoldSansSerif => "bold-sans-serif",
            MathVariant::SansSerifItalic => "sans-serif-italic",
            MathVariant::SansSerifBoldItalic => "sans-serif-bold-italic",
            MathVariant::Monospace => "monospace",
            MathVariant::Initial => "initial",
            MathVariant::Tailed => "tailed",
            MathVariant::Looped => "looped",
            MathVariant::Stretched => "stretched",
        };

        f.write_str(as_str)
    }
}

/// Attribute of a MathML element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Attribute {
    /// Class of the element, same as in HTML.
    Class(String),

    /// Data attribute with custom name and value (e.g. `data-name="value"`).
    Data {
        /// Name of the data attribute, the part after `data-` prefix. e.g. `data-abc="2"` has the
        /// name "abc".
        name: String,

        /// The value of the data attribute. e.g. `data-name="test"` has value "test".
        value: String,
    },

    /// Direction of the math element. Must be either `rtl` (`RightToLeft`) or `ltr`
    /// (`LeftToRight`).
    Dir(Dir),

    /// Display style of the element, `true` maps to `normal` and `false` to `compact`.
    DisplayStyle(bool),

    /// Id of the element, same as in HTML.
    Id(String),

    /// Presentational hint for the background color of the element. Must be a value that is
    /// [color](https://www.w3.org/TR/css-color-4/#propdef-color)
    MathBackground(String),

    /// Presentational hint for the color of the element. Must be a value that is
    /// [color](https://www.w3.org/TR/css-color-4/#propdef-color).
    MathColor(String),

    /// Presentational hint for the font size of the element. Must be a value that is
    /// [length-percentage](https://www.w3.org/TR/css-values-4/#typedef-length-percentage).
    MathSize(String),

    /// The `mathvariant` attribute, if present, must be an ASCII case-insensitive match to one of
    /// [`MathVariant`]. [`MathVariant::Normal`] is mapped to none while any other valid value is
    /// mapped to its ASCII lowercased value, prefixed with `math-`.
    MathVariant(MathVariant),

    /// The `nonce` attribute, same as in HTML.
    Nonce(String),

    /// Presentational hint for setting the element's math-depth property to the corresponding
    /// value. Can be `+<U>` ([`ScriptLevel::Add`]), `-<U>` ([`ScriptLevel::Sub`]) or `<U>`
    /// ([`ScriptLevel::Num`]) where `<U>` is a [`usize`].
    ScriptLevel(ScriptLevel),

    /// The `style` attribute of the element, same as in HTML.
    Style(String),

    /// The `tabindex` attribute, same as in HTML.
    TabIndex(i16),

    /// Event handler function, e.g. `onclick="..."`.
    ///
    /// # Example
    /// ```rust
    /// use alemat::{Attribute, BufMathMlWriter, Writer};
    ///
    /// let handler = Attribute::OnHandler {
    ///     name: "click".to_string(),
    ///     handler: "console.log('Clicked!')".to_string(),
    /// };
    ///
    /// let mut writer = BufMathMlWriter::default();
    /// writer.write_attr(&handler);
    /// let output = writer.finish();
    ///
    /// assert_eq!(output, "onclick=\"console.log('Clicked!')\"");
    OnHandler {
        /// Name of the event.
        name: String,

        /// Handler function for the event.
        handler: String,
    },
}
