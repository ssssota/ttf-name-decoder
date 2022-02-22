# ttf-name-decoder

Decoder for names of truetype fonts.

## Usage

```rs
let platform_id: u16 = 3; // Windows
let encoding_id: u16 = 1; // Unicode
let language_id: u16 = 1033; // English
let name_data: Vec<u8> = get_data();
let result = ttf_name_decoder::decode(&name_data)?;
```
