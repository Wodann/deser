use super::DeserializationError;

/// Provides a `Visitor` access to each element of a sequence in the input.
pub trait SequenceAccessor {
    type Error: DeserializationError;
}
