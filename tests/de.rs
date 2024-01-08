use deser::de::{custom::CustomDeserializer, Deserialize, Visitor};

#[test]
fn test() {
    #[derive(Debug)]
    struct LimitedU32(u32);

    #[derive(Debug, PartialEq, Eq)]
    enum LimitedU32Error {
        TooLarge,
    }

    struct LimitedU32Visitor;

    impl Visitor for LimitedU32Visitor {
        type Error = LimitedU32Error;

        type Value = LimitedU32;

        fn visit_u32(self, v: u32) -> Result<Self::Value, Self::Error> {
            if v > 100 {
                Err(LimitedU32Error::TooLarge)
            } else {
                Ok(LimitedU32(v))
            }
        }

        fn visit_u64(self, v: u64) -> Result<Self::Value, Self::Error> {
            if v > 100 {
                Err(LimitedU32Error::TooLarge)
            } else {
                Ok(LimitedU32(v as u32))
            }
        }
    }

    impl Deserialize for LimitedU32 {
        type Error = LimitedU32Error;

        fn deserialize<DeserializerT>(
            deserializer: DeserializerT,
        ) -> Result<Self, Error<DeserializerT::Error, Self::Error>>
        where
            DeserializerT: Deserializer,
        {
            deserializer.deserialize_u32(LimitedU32Visitor)
        }
    }

    #[derive(Debug)]
    struct LimitedU64(u64);

    #[derive(Debug, PartialEq, Eq)]
    enum LimitedU64Error {
        TooLarge,
    }

    struct LimitedU64Visitor;

    impl Visitor for LimitedU64Visitor {
        type Error = LimitedU64Error;

        type Value = LimitedU64;

        fn visit_u32(self, v: u32) -> Result<Self::Value, Self::Error> {
            if v > 100 {
                Err(LimitedU64Error::TooLarge)
            } else {
                Ok(LimitedU64(v as u64))
            }
        }

        fn visit_u64(self, v: u64) -> Result<Self::Value, Self::Error> {
            if v > 100 {
                Err(LimitedU64Error::TooLarge)
            } else {
                Ok(LimitedU64(v))
            }
        }
    }

    impl Deserialize for LimitedU64 {
        type Error = LimitedU64Error;

        fn deserialize<DeserializerT>(
            deserializer: DeserializerT,
        ) -> Result<Self, Error<DeserializerT::Error, Self::Error>>
        where
            DeserializerT: Deserializer,
        {
            deserializer.deserialize_u32(LimitedU64Visitor)
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    enum FooError {
        LimitedU32Error(LimitedU32Error),
        LimitedU64Error(LimitedU64Error),
        InvalidInput,
    }

    impl From<LimitedU32Error> for FooError {
        fn from(e: LimitedU32Error) -> Self {
            FooError::LimitedU32Error(e)
        }
    }

    impl From<LimitedU64Error> for FooError {
        fn from(e: LimitedU64Error) -> Self {
            FooError::LimitedU64Error(e)
        }
    }

    impl<DeserializerErrorT> From<FooError> for Error<DeserializerErrorT, FooError> {
        fn from(e: FooError) -> Self {
            Error::Visitor(e)
        }
    }

    #[derive(Debug)]
    struct Foo {
        a: LimitedU32,
        b: LimitedU64,
    }

    impl Deserialize for Foo {
        type Error = FooError;

        fn deserialize<DeserializerT>(
            deserializer: DeserializerT,
        ) -> Result<Self, Error<DeserializerT::Error, Self::Error>>
        where
            DeserializerT: Deserializer,
        {
            struct FooVisitor;

            impl Visitor for FooVisitor {
                type Error = FooError;

                type Value = Foo;

                fn visit_u32(self, _v: u32) -> Result<Self::Value, Self::Error> {
                    Err(FooError::InvalidInput)
                }

                fn visit_u64(self, _v: u64) -> Result<Self::Value, Self::Error> {
                    Err(FooError::InvalidInput)
                }

                fn visit_u32_and_u64(self, v: u32, v2: u64) -> Result<Self::Value, Self::Error> {
                    Ok(Foo {
                        a: LimitedU32Visitor.visit_u32(v)?,
                        b: LimitedU64Visitor.visit_u64(v2)?,
                    })
                }
            }

            deserializer.deserialize_u32_and_u64(FooVisitor)
        }
    }

    let mut deserializer = custom::CustomDeserializer::new("97 456");
    let error = Foo::deserialize(&mut deserializer).expect_err("Should fail with visitor error");

    assert_eq!(
        error,
        Error::Visitor(FooError::LimitedU64Error(LimitedU64Error::TooLarge))
    );

    let mut deserializer = custom::CustomDeserializer::new("97 68");
    let foo = Foo::deserialize(&mut deserializer).expect("Should succeed");

    assert_eq!(foo.a.0, 97);
    assert_eq!(foo.b.0, 68);
}
