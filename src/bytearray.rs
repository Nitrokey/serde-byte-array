use core::array::TryFromSliceError;
use core::borrow::{Borrow, BorrowMut};
use core::cmp::Ordering;
use core::convert::TryInto;
use core::fmt::{self, Debug};
use core::hash::{Hash, Hasher};
use core::mem;
use core::ops::{Deref, DerefMut};

use serde::de::{Deserialize, Deserializer, Error, SeqAccess, Visitor};
use serde::ser::{Serialize, Serializer};

/// Wrapper around `[u8; N]` to serialize and deserialize efficiently.
///
/// ```
/// use std::collections::HashMap;
/// use std::io;
///
/// use serde_byte_array::ByteArray;
///
/// fn deserialize_bytearrays() -> bincode::Result<()> {
///     let example_data = [
///         2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 116,
///         119, 111, 1, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 111, 110, 101
///     ];
///
///     let map: HashMap<u32, ByteArray<3>> = bincode::deserialize(&example_data[..])?;
///
///     println!("{:?}", map);
///
///     Ok(())
/// }
/// #
/// # fn main() {
/// #     deserialize_bytearrays().unwrap();
/// # }
/// ```
#[derive(Copy, Clone, Eq, Ord)]
#[repr(transparent)]
pub struct ByteArray<const N: usize> {
    bytes: [u8; N],
}

impl<const N: usize> ByteArray<N> {
    /// Transform an [array](https://doc.rust-lang.org/stable/std/primitive.array.html) to the equivalent `ByteArray`
    pub const fn new(bytes: [u8; N]) -> Self {
        Self { bytes }
    }

    /// Wrap existing bytes into a `ByteArray`
    pub fn from<T: Into<[u8; N]>>(bytes: T) -> Self {
        Self {
            bytes: bytes.into(),
        }
    }

    /// Return a slice containing all bytes.
    pub const fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    /// Unwraps the byte array underlying this `ByteArray`
    pub const fn into_array(self) -> [u8; N] {
        self.bytes
    }
}

impl<const N: usize> From<[u8; N]> for ByteArray<N> {
    fn from(bytes: [u8; N]) -> ByteArray<N> {
        ByteArray { bytes }
    }
}

impl<const N: usize> From<ByteArray<N>> for [u8; N] {
    fn from(bytes: ByteArray<N>) -> [u8; N] {
        bytes.bytes
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for &'a ByteArray<N> {
    fn from(bytes: &'a [u8; N]) -> &'a ByteArray<N> {
        // Safety: #[repr(transparent)]
        unsafe { mem::transmute::<&'a [u8; N], &'a ByteArray<N>>(bytes) }
    }
}

impl<'a, const N: usize> From<&'a ByteArray<N>> for &'a [u8; N] {
    fn from(bytes: &'a ByteArray<N>) -> &'a [u8; N] {
        // Safety: #[repr(transparent)]
        unsafe { mem::transmute::<&'a ByteArray<N>, &'a [u8; N]>(bytes) }
    }
}

impl<'a, const N: usize> TryFrom<&'a [u8]> for ByteArray<N> {
    type Error = TryFromSliceError;
    fn try_from(bytes: &'a [u8]) -> Result<Self, TryFromSliceError> {
        Ok(Self {
            bytes: bytes.try_into()?,
        })
    }
}

impl<'a, const N: usize> TryFrom<&'a [u8]> for &'a ByteArray<N> {
    type Error = TryFromSliceError;
    fn try_from(bytes: &'a [u8]) -> Result<Self, TryFromSliceError> {
        let tmp: &'a [u8; N] = bytes.try_into()?;
        Ok(tmp.into())
    }
}

impl<const N: usize> Debug for ByteArray<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.bytes, f)
    }
}

impl<const N: usize> Default for ByteArray<N> {
    fn default() -> Self {
        Self::new([Default::default(); N])
    }
}

impl<const N: usize> AsRef<[u8; N]> for ByteArray<N> {
    fn as_ref(&self) -> &[u8; N] {
        &self.bytes
    }
}
impl<const N: usize> AsMut<[u8; N]> for ByteArray<N> {
    fn as_mut(&mut self) -> &mut [u8; N] {
        &mut self.bytes
    }
}

impl<const N: usize> AsRef<[u8]> for ByteArray<N> {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl<const N: usize> AsMut<[u8]> for ByteArray<N> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}

impl<const N: usize> Borrow<[u8; N]> for ByteArray<N> {
    fn borrow(&self) -> &[u8; N] {
        &self.bytes
    }
}
impl<const N: usize> BorrowMut<[u8; N]> for ByteArray<N> {
    fn borrow_mut(&mut self) -> &mut [u8; N] {
        &mut self.bytes
    }
}

impl<const N: usize> Borrow<[u8]> for ByteArray<N> {
    fn borrow(&self) -> &[u8] {
        &self.bytes
    }
}
impl<const N: usize> BorrowMut<[u8]> for ByteArray<N> {
    fn borrow_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}

impl<const N: usize> Deref for ByteArray<N> {
    type Target = [u8; N];

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl<const N: usize> DerefMut for ByteArray<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bytes
    }
}

impl<Rhs, const N: usize> PartialEq<Rhs> for ByteArray<N>
where
    Rhs: ?Sized + Borrow<[u8; N]>,
{
    fn eq(&self, other: &Rhs) -> bool {
        (**self).eq(other.borrow())
    }
}

impl<Rhs, const N: usize> PartialOrd<Rhs> for ByteArray<N>
where
    Rhs: ?Sized + Borrow<[u8; N]>,
{
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering> {
        (**self).partial_cmp(other.borrow())
    }
}

impl<const N: usize> Hash for ByteArray<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bytes.hash(state);
    }
}

impl<const N: usize> IntoIterator for ByteArray<N> {
    type Item = u8;
    type IntoIter = <[u8; N] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.bytes)
    }
}

impl<'a, const N: usize> IntoIterator for &'a ByteArray<N> {
    type Item = &'a u8;
    type IntoIter = <&'a [u8; N] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes.iter()
    }
}

impl<'a, const N: usize> IntoIterator for &'a mut ByteArray<N> {
    type Item = &'a mut u8;
    type IntoIter = <&'a mut [u8; N] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes.iter_mut()
    }
}

impl<const N: usize> Serialize for ByteArray<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.bytes)
    }
}

struct ByteArrayVisitor<const N: usize>;

impl<'de, const N: usize> Visitor<'de> for ByteArrayVisitor<N> {
    type Value = ByteArray<N>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a byte array of length {}", N)
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<ByteArray<N>, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut bytes = [0; N];

        for (idx, byte) in bytes.iter_mut().enumerate() {
            *byte = seq
                .next_element()?
                .ok_or_else(|| V::Error::invalid_length(idx, &self))?;
        }

        Ok(ByteArray::from(bytes))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<ByteArray<N>, E>
    where
        E: Error,
    {
        Ok(ByteArray {
            bytes: v
                .try_into()
                .map_err(|_| E::invalid_length(v.len(), &self))?,
        })
    }

    fn visit_str<E>(self, v: &str) -> Result<ByteArray<N>, E>
    where
        E: Error,
    {
        self.visit_bytes(v.as_bytes())
    }
}

impl<'de, const N: usize> Deserialize<'de> for ByteArray<N> {
    fn deserialize<D>(deserializer: D) -> Result<ByteArray<N>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(ByteArrayVisitor::<N>)
    }
}

struct ByteArrayRefVisitor<const N: usize>;

impl<'de, const N: usize> Visitor<'de> for ByteArrayRefVisitor<N> {
    type Value = &'de ByteArray<N>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a byte array of length {}", N)
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<&'de ByteArray<N>, E>
    where
        E: Error,
    {
        let arr: &[u8; N] = v
            .try_into()
            .map_err(|_| E::invalid_length(v.len(), &self))?;
        Ok(arr.into())
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<&'de ByteArray<N>, E>
    where
        E: Error,
    {
        self.visit_borrowed_bytes(v.as_bytes())
    }
}

impl<'de, const N: usize> Deserialize<'de> for &'de ByteArray<N> {
    fn deserialize<D>(deserializer: D) -> Result<&'de ByteArray<N>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(ByteArrayRefVisitor)
    }
}
