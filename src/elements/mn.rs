use crate::attributes::Attribute;

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

impl Num {
    pub fn add_attr<I, A>(&mut self, attr: I)
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        self.attributes.extend(attr.into_iter().map(Into::into));
    }

    pub fn with_attr<I, A>(mut self, attr: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        self.attributes.extend(attr.into_iter().map(Into::into));
        self
    }

    pub fn num(&self) -> &str {
        &self.num
    }

    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

crate::element_from_type!(Num => Num);
