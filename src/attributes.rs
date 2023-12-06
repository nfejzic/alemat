use std::fmt::Display;

/// Direction for [`Attribute::Dir`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dir {
    RightToLeft,
    LeftToRight,
}

/// ScriptLevel for [`Attribute::ScriptLevel`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScriptLevel {
    Add(usize),
    Sub(usize),
    Num(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MathVariant {
    Normal,
    Bold,
    Italic,
    BoldItalic,
    DoubleStruck,
    BoldFraktur,
    Script,
    BoldScript,
    Fraktur,
    SansSerif,
    BoldSansSerif,
    SansSerifItalic,
    SansSerifBoldItalic,
    Monospace,
    Initial,
    Tailed,
    Looped,
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

    OnHandler {
        /// Name of the event.
        name: String,

        /// Handler function for the event.
        handler: String,
    },
}
