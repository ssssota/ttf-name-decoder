use std::convert::{Into, TryFrom};

/// ref. <https://docs.microsoft.com/en-us/typography/opentype/spec/name#platform-ids>
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PlatformId {
    Unicode,
    Macintosh,
    ISO,
    Windows,
    Custom,
}

impl TryFrom<u16> for PlatformId {
    type Error = &'static str;
    fn try_from(id: u16) -> Result<Self, Self::Error> {
        match id {
            0 => Ok(PlatformId::Unicode),
            1 => Ok(PlatformId::Macintosh),
            2 => Ok(PlatformId::ISO),
            3 => Ok(PlatformId::Windows),
            4 => Ok(PlatformId::Custom),
            _ => Err("Invalid Platform ID"),
        }
    }
}

impl Into<u16> for PlatformId {
    fn into(self) -> u16 {
        match self {
            PlatformId::Unicode => 0,
            PlatformId::Macintosh => 1,
            PlatformId::ISO => 2,
            PlatformId::Windows => 3,
            PlatformId::Custom => 4,
        }
    }
}
