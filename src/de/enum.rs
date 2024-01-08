use super::DeserializationError;

pub trait EnumAccessor {
    type Error: DeserializationError;
}
