pub trait ToMathMl {
    /// Generate a MathML representation of the element.
    fn to_mathml(&self) -> String;

    /// Generate the style attribute for the element. Not all elements need a style, so this is
    /// optionally implemented.
    fn style(&self) -> Option<String> {
        None
    }
}
