use super::DeserializationError;

pub trait MapAccessor {
    type Error: DeserializationError;
}
