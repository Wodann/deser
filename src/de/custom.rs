use super::{DeserializationError, Deserializer, Error, Expected, Unexpected, Visitor};

pub struct CustomDeserializer<'a> {
    input: &'a str,
}

impl<'a> CustomDeserializer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CustomError {
    MissingComma,
    ParseIntError {
        error: std::num::ParseIntError,
        input: String,
    },
}

impl DeserializationError for CustomError {
    fn invalid_type(actual: Unexpected<'_>, expected: &impl Expected) -> Self {
        todo!()
    }

    fn invalid_value(actual: Unexpected<'_>, expected: &impl Expected) -> Self {
        todo!()
    }

    fn invalid_length(actual: usize, expected: &impl Expected) -> Self {
        todo!()
    }
}

impl<'a> Deserializer for &'a mut CustomDeserializer<'_> {
    type Error = CustomError;

    fn deserialize_u32<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor,
    {
        let mut end = 0;
        for (i, c) in self.input.char_indices() {
            if c.is_ascii_digit() {
                end = i + 1;
            } else {
                break;
            }
        }
        let number = self.input[..end]
            .parse()
            .map_err(|error| CustomError::ParseIntError {
                error,
                input: self.input[..end].to_string(),
            })
            .map_err(Error::Deserializer)?;

        self.input = &self.input[end..];
        visitor.visit_u32(number).map_err(Error::Visitor)
    }

    fn deserialize_u64<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor,
    {
        let mut end = 0;
        for (i, c) in self.input.char_indices() {
            if c.is_ascii_digit() {
                end = i + 1;
            } else {
                break;
            }
        }
        let number = self.input[..end]
            .parse()
            .map_err(|error| CustomError::ParseIntError {
                error,
                input: self.input[..end].to_string(),
            })
            .map_err(Error::Deserializer)?;

        self.input = &self.input[end..];
        visitor.visit_u64(number).map_err(Error::Visitor)
    }

    fn deserialize_u32_and_u64<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor,
    {
        let mut end = 0;
        for (i, c) in self.input.char_indices() {
            if c.is_ascii_digit() {
                end = i + 1;
            } else {
                break;
            }
        }
        let number1 = self.input[..end]
            .parse()
            .map_err(|error| CustomError::ParseIntError {
                error,
                input: self.input[..end].to_string(),
            })
            .map_err(Error::Deserializer)?;

        self.input = &self.input[end..];

        if !self.input.starts_with(' ') {
            return Err(Error::Deserializer(CustomError::MissingComma));
        }

        self.input = &self.input[1..];

        let mut end = 0;
        for (i, c) in self.input.char_indices() {
            if c.is_ascii_digit() {
                end = i + 1;
            } else {
                break;
            }
        }
        let number2 = self.input[..end]
            .parse()
            .map_err(|error| CustomError::ParseIntError {
                error,
                input: self.input[..end].to_string(),
            })
            .map_err(Error::Deserializer)?;

        self.input = &self.input[end..];

        visitor
            .visit_u32_and_u64(number1, number2)
            .map_err(Error::Visitor)
    }
}
