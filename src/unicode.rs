use crate::macros::generate_id_table;
// ref. https://docs.microsoft.com/en-us/typography/opentype/spec/name#platform-specific-encoding-and-language-ids-unicode-platform-platform-id--0
generate_id_table! {
    EncodingId,
    Unicode1_0: 0,
    Unicode1_1: 1,
    IsoIec10646: 2,
    UnicodeBMP: 3,
    UnicodeFull: 4,
}
