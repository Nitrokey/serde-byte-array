use serde_byte_array::ByteArray;
use serde_test::{assert_de_tokens, assert_ser_tokens, assert_tokens, Token};

#[test]
fn test_bytearray() {
    let empty = ByteArray::new([]);
    assert_tokens(&empty, &[Token::BorrowedBytes(b"")]);
    assert_ser_tokens(&empty, &[Token::Bytes(b"")]);
    assert_ser_tokens(&empty, &[Token::ByteBuf(b"")]);
    assert_de_tokens(&empty, &[Token::BorrowedStr("")]);

    let buf = [65, 66, 67];
    let bytes = ByteArray::new(buf);
    assert_tokens(&bytes, &[Token::BorrowedBytes(b"ABC")]);
    assert_ser_tokens(&bytes, &[Token::Bytes(b"ABC")]);
    assert_ser_tokens(&bytes, &[Token::ByteBuf(b"ABC")]);
    assert_de_tokens(&bytes, &[Token::BorrowedStr("ABC")]);
}
