use crate::macros::generate_decoder;

// Unicode (UTF-16 BE)
generate_decoder!(utf16_be_decode, encoding_rs::UTF_16BE);
// Basic for legacy macintosh
generate_decoder!(mac_roman_decode, encoding_rs::MACINTOSH);
// Japanese
generate_decoder!(shift_jis_decode, encoding_rs::SHIFT_JIS);
// Chinese
generate_decoder!(big5_decode, encoding_rs::BIG5);
generate_decoder!(gbk_decode, encoding_rs::GBK);
generate_decoder!(gb18030_decode, encoding_rs::GB18030);
// Korean
generate_decoder!(euc_kr_decode, encoding_rs::EUC_KR);
// Arabic
generate_decoder!(iso_8859_6_decode, encoding_rs::ISO_8859_6);
// Greek
generate_decoder!(iso_8859_7_decode, encoding_rs::ISO_8859_7);
// Hebrew
generate_decoder!(iso_8859_8_decode, encoding_rs::ISO_8859_8);
// Russian
generate_decoder!(cyrillic_decode, encoding_rs::X_MAC_CYRILLIC);
