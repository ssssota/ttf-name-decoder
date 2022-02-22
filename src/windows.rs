use crate::macros::generate_id_table;
use crate::{decoders, Decoder, Error, Result};

pub fn get_decoder(encoding_id: u16) -> Result<Decoder> {
    match EncodingId::try_from(encoding_id)? {
        EncodingId::Symbol => Err(Error::UnsupportedEncoding),
        EncodingId::UnicodeBMP => Ok(decoders::utf16_be_decode),
        EncodingId::ShiftJIS => Ok(decoders::shift_jis_decode),
        EncodingId::PRC => Ok(decoders::gb18030_decode),
        EncodingId::Big5 => Ok(decoders::big5_decode),
        EncodingId::Wansung => Err(Error::UnsupportedEncoding),
        EncodingId::Johab => Err(Error::UnsupportedEncoding),
        EncodingId::UnicodeFull => Ok(decoders::utf16_be_decode),
    }
}

// ref. https://docs.microsoft.com/en-us/typography/opentype/spec/name#platform-ids
generate_id_table! (EncodingId, Error::UnsupportedEncoding, {
    Symbol: 0,
    UnicodeBMP: 1,
    ShiftJIS: 2,
    PRC: 3,
    Big5: 4,
    Wansung: 5,
    Johab: 6,
    UnicodeFull: 10,
});
