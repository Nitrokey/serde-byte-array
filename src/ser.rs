use crate::ByteArray;
use serde::Serializer;

/// Types that can be serialized via `#[serde(with = "serde_byte_array")]`.
pub trait Serialize {
    #[allow(missing_docs)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

impl<const N: usize> Serialize for [u8; N] {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(self)
    }
}

impl<const N: usize> Serialize for ByteArray<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&**self)
    }
}

impl<'a, T: Serialize> Serialize for &'a T {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self).serialize(serializer)
    }
}

impl<T> Serialize for Option<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        struct AsBytes<T>(T);

        impl<T> serde::Serialize for AsBytes<T>
        where
            T: Serialize,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                self.0.serialize(serializer)
            }
        }

        match self {
            Some(b) => serializer.serialize_some(&AsBytes(b)),
            None => serializer.serialize_none(),
        }
    }
}
