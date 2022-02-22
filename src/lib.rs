//! This crate create decoder for ttf name record.
//!
//! # Usage
//!
//! ```rs
//! let platform_id: u16 = 3; // Windows
//! let encoding_id: u16 = 1; // Unicode
//! let language_id: u16 = 1033; // English
//! let name_data: Vec<u8> = get_data();
//! let result = ttf_name_decoder::decode(&name_data)?;
//! ```

mod decoders;
pub mod macintosh;
mod macros;
pub mod platform;
pub mod unicode;
pub mod windows;

pub type Decoder = fn(data: &[u8]) -> Option<String>;
#[derive(Debug, PartialEq)]
pub enum Error {
    UnsupportedPlatform,
    UnsupportedEncoding,
    UnsupportedLanguage,
    CannotDecode,
}
pub type Result<T> = std::result::Result<T, Error>;

pub fn decode(data: &[u8], platform_id: u16, encoding_id: u16, language_id: u16) -> Result<String> {
    let d = get_decoder(platform_id, encoding_id, language_id)?;
    d(data).ok_or(Error::CannotDecode)
}

fn get_decoder(platform_id: u16, encoding_id: u16, language_id: u16) -> Result<Decoder> {
    use platform::PlatformId;
    match PlatformId::try_from(platform_id)? {
        PlatformId::Unicode => Ok(decoders::utf16_be_decode),
        PlatformId::Macintosh => macintosh::get_decoder(encoding_id, language_id),
        PlatformId::ISO => Err(Error::UnsupportedPlatform),
        PlatformId::Windows => windows::get_decoder(encoding_id),
        PlatformId::Custom => Err(Error::UnsupportedPlatform),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn unicode() {
        let data = vec![0x00, 0x74, 0x00, 0x65, 0x00, 0x73, 0x00, 0x74];
        let result = super::decode(&data, 0, 0, 0).unwrap();
        assert_eq!("test", result);
    }
    #[test]
    fn windows_japanese() {
        let data = vec![131, 101, 131, 88, 131, 103];
        let result = super::decode(&data, 3, 2, 0).unwrap();
        assert_eq!("テスト", result);
    }
    #[test]
    fn unsupported_platform() {
        let data = vec![];
        let err = super::decode(&data, 7, 0, 0).unwrap_err();
        assert_eq!(super::Error::UnsupportedPlatform, err);
    }
}
