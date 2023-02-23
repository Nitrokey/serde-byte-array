use serde::{Deserialize, Serialize};
use serde_byte_array::ByteArray;
use serde_test::{assert_tokens, Token};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Test<'a> {
    #[serde(with = "serde_byte_array")]
    array: [u8; 314],

    #[serde(with = "serde_byte_array", borrow)]
    array_ref: &'a [u8; 314],

    #[serde(with = "serde_byte_array")]
    byte_array: ByteArray<314>,

    #[serde(with = "serde_byte_array", borrow)]
    byte_array_ref: &'a ByteArray<314>,

    #[serde(with = "serde_byte_array")]
    opt_array: Option<[u8; 314]>,

    #[serde(with = "serde_byte_array")]
    opt_bytearray: Option<ByteArray<314>>,
}
#[test]
fn test() {
    let test = Test {
        array: [0; 314],

        array_ref: &[1; 314],

        byte_array: [2u8; 314].into(),

        byte_array_ref: (&[3; 314]).into(),

        opt_array: Some([4; 314]),

        opt_bytearray: Some([5u8; 314].into()),
    };

    assert_tokens(
        &test,
        &[
            Token::Struct {
                name: "Test",
                len: 6,
            },
            Token::Str("array"),
            Token::Bytes(&[0; 314]),
            Token::Str("array_ref"),
            Token::BorrowedBytes(&[1; 314]),
            Token::Str("byte_array"),
            Token::Bytes(&[2; 314]),
            Token::Str("byte_array_ref"),
            Token::BorrowedBytes(&[3; 314]),
            Token::Str("opt_array"),
            Token::Some,
            Token::Bytes(&[4; 314]),
            Token::Str("opt_bytearray"),
            Token::Some,
            Token::Bytes(&[5; 314]),
            Token::StructEnd,
        ],
    );
}
