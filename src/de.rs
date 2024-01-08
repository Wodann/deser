pub mod custom;
mod r#enum;
mod error;
pub mod json;
mod map;
mod sequence;
mod visitor;

pub use self::{error::*, map::*, r#enum::*, sequence::*, visitor::*};

pub trait Deserialize: Sized {
    type Error;

    fn deserialize<DeserializerT>(
        deserializer: DeserializerT,
    ) -> Result<Self, Error<DeserializerT::Error, Self::Error>>
    where
        DeserializerT: Deserializer;
}

pub trait Deserializer: Sized {
    type Error: DeserializationError;

    fn deserialize_bool<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_i8<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_i16<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_i32<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_i64<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_u8<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_u16<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_u32<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_u64<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_f32<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_f64<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_char<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_str<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_string<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_bytes<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_byte_buf<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_option<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_unit<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_unit_struct<VisitorT>(
        self,
        name: &'static str,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_newtype_struct<VisitorT>(
        self,
        name: &'static str,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_sequence<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_tuple<VisitorT>(
        self,
        length: usize,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_tuple_struct<VisitorT>(
        self,
        name: &'static str,
        length: usize,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_map<VisitorT>(
        self,
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_struct<VisitorT>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;

    fn deserialize_enum<VisitorT>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: VisitorT,
    ) -> Result<VisitorT::Value, Error<Self::Error, VisitorT::Error>>
    where
        VisitorT: Visitor;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error<DeserializerErrorT, VisitorErrorT> {
    Deserializer(DeserializerErrorT),
    Visitor(VisitorErrorT),
}

impl<DeserializerErrorT, VisitorErrorT> Error<DeserializerErrorT, VisitorErrorT> {
    pub fn map_visitor_err<ErrorT>(self) -> Error<DeserializerErrorT, ErrorT>
    where
        VisitorErrorT: Into<ErrorT>,
    {
        match self {
            Error::Deserializer(e) => Error::Deserializer(e),
            Error::Visitor(e) => Error::Visitor(e.into()),
        }
    }
}
