use core::fmt;

use super::{
    DeserializationError, Deserializer, EnumAccessor, MapAccessor, SequenceAccessor, Unexpected,
};

pub trait Visitor: Sized {
    type Error: DeserializationError;

    type Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result;

    fn visit_bool(self, value: bool) -> Result<Self::Value, Self::Error> {
        Err(Self::Error::invalid_type(Unexpected::Bool(value), &self))
    }

    fn visit_i8(self, value: i8) -> Result<Self::Value, Self::Error> {
        self.visit_i64(value.into())
    }

    fn visit_i16(self, value: i16) -> Result<Self::Value, Self::Error> {
        self.visit_i64(value.into())
    }

    fn visit_i32(self, value: i32) -> Result<Self::Value, Self::Error> {
        self.visit_i64(value.into())
    }

    fn visit_i64(self, value: i64) -> Result<Self::Value, Self::Error> {
        Err(Self::Error::invalid_type(Unexpected::Signed(value), &self))
    }

    fn visit_u8(self, value: u8) -> Result<Self::Value, Self::Error> {
        self.visit_u64(value.into())
    }

    fn visit_u16(self, value: u16) -> Result<Self::Value, Self::Error> {
        self.visit_u64(value.into())
    }

    fn visit_u32(self, value: u32) -> Result<Self::Value, Self::Error> {
        self.visit_u64(value.into())
    }

    fn visit_u64(self, value: u64) -> Result<Self::Value, Self::Error> {
        Err(Self::Error::invalid_type(
            Unexpected::Unsigned(value),
            &self,
        ))
    }

    fn visit_f32(self, value: f32) -> Result<Self::Value, Self::Error> {
        self.visit_f64(value.into())
    }

    fn visit_f64(self, value: f64) -> Result<Self::Value, Self::Error> {
        Err(Self::Error::invalid_type(Unexpected::Float(value), &self))
    }

    fn visit_char(self, value: char) -> Result<Self::Value, Self::Error> {
        self.visit_str(value.encode_utf8(&mut [0u8; 4]))
    }

    fn visit_str(self, value: &str) -> Result<Self::Value, Self::Error> {
        Err(Self::Error::invalid_type(Unexpected::Str(value), &self))
    }

    fn visit_string(self, value: String) -> Result<Self::Value, Self::Error> {
        self.visit_str(&value)
    }

    fn visit_bytes(self, value: &[u8]) -> Result<Self::Value, Self::Error> {
        Err(Self::Error::invalid_type(Unexpected::Bytes(value), &self))
    }

    fn visit_bytes_buf(self, value: Vec<u8>) -> Result<Self::Value, Self::Error> {
        self.visit_bytes(&value)
    }

    fn visit_none(self) -> Result<Self::Value, Self::Error> {
        Err(Self::Error::invalid_type(Unexpected::Option, &self))
    }

    fn visit_some<DeserializerT>(
        self,
        deserializer: DeserializerT,
    ) -> Result<Self::Value, Self::Error>
    where
        DeserializerT: Deserializer,
        DeserializerT::Error: DeserializationError + Into<Self::Error>,
    {
        let _ = deserializer;
        Err(Self::Error::invalid_type(Unexpected::Option, &self))
    }

    fn visit_unit(self) -> Result<Self::Value, Self::Error> {
        Err(Self::Error::invalid_type(Unexpected::Unit, &self))
    }

    fn visit_newtype_struct<DeserializerT>(
        self,
        deserializer: DeserializerT,
    ) -> Result<Self::Value, Self::Error>
    where
        DeserializerT: Deserializer,
        DeserializerT::Error: DeserializationError + Into<Self::Error>,
    {
        let _ = deserializer;
        Err(Self::Error::invalid_type(Unexpected::NewtypeStruct, &self))
    }

    fn visit_sequence<AccessorT>(self, accessor: AccessorT) -> Result<Self::Value, Self::Error>
    where
        AccessorT: SequenceAccessor,
    {
        let _ = accessor;
        Err(Self::Error::invalid_type(Unexpected::Sequence, &self))
    }

    fn visit_map<AccessorT>(self, accessor: AccessorT) -> Result<Self::Value, Self::Error>
    where
        AccessorT: MapAccessor,
    {
        let _ = accessor;
        Err(Self::Error::invalid_type(Unexpected::Sequence, &self))
    }

    fn visit_enum<AccessorT>(self, accessor: AccessorT) -> Result<Self::Value, Self::Error>
    where
        AccessorT: EnumAccessor,
    {
        let _ = accessor;
        Err(Self::Error::invalid_type(Unexpected::Sequence, &self))
    }
}
