#![no_std]

use serde::{Deserializer, Serializer};

mod bytearray;
mod de;
mod ser;

pub use bytearray::ByteArray;
pub use de::Deserialize;
pub use ser::Serialize;

/// Serde `serialize_with` function to serialize bytes efficiently.
///
/// This function can be used with either of the following Serde attributes:
///
/// - `#[serde(with = "serde_byte_array")]`
/// - `#[serde(serialize_with = "serde_byte_array::serialize")]`
///
/// ```
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Efficient<'a> {
///     #[serde(with = "serde_byte_array")]
///     byte_array: [u8; 314],

///     #[serde(with = "serde_byte_array")]
///     byte_array_ref: &'a [u8; 314],
/// }
/// ```
pub fn serialize<T, S>(bytes: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ?Sized + Serialize,
    S: Serializer,
{
    Serialize::serialize(bytes, serializer)
}

/// Serde `deserialize_with` function to deserialize bytes efficiently.
///
/// This function can be used with either of the following Serde attributes:
///
/// - `#[serde(with = "serde_byte_array")]`
/// - `#[serde(deserialize_with = "serde_byte_array::deserialize")]`
///
/// ```
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Packet {
///     #[serde(with = "serde_byte_array")]
///     byte_array: [u8; 314],
/// }
/// ```
pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer)
}
