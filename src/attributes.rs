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
