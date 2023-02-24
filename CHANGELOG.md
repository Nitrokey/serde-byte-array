# Changelog

## [v0.1.2][] (2023-27-02)

- Implemet `Copy` for ByteArray ([#5][])

[#5]: https://github.com/Nitrokey/serde-byte-array/pull/5
[v0.1.1]: https://github.com/Nitrokey/serde-byte-array/releases/tag/v0.1.2

## [v0.1.1][] (2023-24-02)

- Improve compatibility with slices ([#4][])
  - Implement `TryFrom<&[u8]>` for `ByteArray` and `&ByteArray`
  - Implement `AsRef<[u8]>` and `AsMut<[u8]>` for `ByteArray`
  - Implement `Borrow<[u8]>` and `BorrowMut<[u8]>` for `ByteArray`

[#4]: https://github.com/Nitrokey/serde-byte-array/pull/4
[v0.1.1]: https://github.com/Nitrokey/serde-byte-array/releases/tag/v0.1.1

## [v0.1.0][] (2023-24-02)

Initial release

[v0.1.0]: https://github.com/Nitrokey/serde-byte-array/releases/tag/v0.1.0