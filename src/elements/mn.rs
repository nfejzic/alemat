use crate::{attributes::Attribute, MathMl, Tag};

/// The `mn` element represents a "numeric literal" or other data that should be rendered as a
/// numeric literal. Generally speaking, a numeric literal is a sequence of digits, perhaps
/// including a decimal point, representing an unsigned integer or real number.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Num {
    num: String,
    attributes: Vec<Attribute>,
}

impl<'a> From<&'a str> for Num {
    fn from(value: &'a str) -> Self {
        Self {
            num: String::from(value),
            attributes: Default::default(),
        }
    }
}

crate::from_types!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize => Num;
            |val| Num { num: format!("{}", val), attributes: Default::default() });

impl<T> From<T> for MathMl
where
    T: Into<Num>,
{
    fn from(value: T) -> Self {
        MathMl {
            content: vec![Tag::Num(value.into())],
        }
    }
}
