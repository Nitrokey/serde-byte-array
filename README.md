# serde\_byte\_array

Wrapper types to enable optimized handling of `[u8; N]`

## Explanation

Without specialization, Rust forces Serde to treat `[u8; N]` just like any
other array. In reality this particular array can often be serialized and
deserialized in a more efficient, compact representation in many formats.

When working with such a format, you can opt into specialized handling of
`[u8; N]` by wrapping it in `serde_byte_array::ByteArray<N>`.

Additionally this crate supports the Serde `with` attribute to enable efficient
handling of `[u8; N]` and `&[u8; N]`  in structs without needing a wrapper type.

## Example

```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Efficient<'a> {
    #[serde(with = "serde_byte_array")]
    bytes: [u8; 3],

    #[serde(with = "serde_byte_array", borrow)]
    bytes_ref: &'a [u8; 3],
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
