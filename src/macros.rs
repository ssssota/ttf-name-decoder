#[macro_export]
macro_rules! generate_decoder {
    ($name:ident, $encoding:expr) => {
        pub fn $name(data: &[u8]) -> Option<String> {
            Some($encoding.decode(data).0.into_owned())
        }
    };
}

pub use generate_decoder;

#[macro_export]
macro_rules! generate_id_table {
    ( $enum:ident, $err:expr, { $( $name:ident : $id:expr ,)+ } ) => {
        #[derive(Debug, PartialEq, Clone, Copy)]
        pub enum $enum {
            $($name,)*
        }

        impl std::convert::TryFrom<u16> for $enum {
            type Error = $crate::Error;
            fn try_from(id: u16) -> $crate::Result<Self> {
                match id {
                    $( $id => Ok($enum::$name) ,)*
                    _ => Err($err),
                }
            }
        }
        impl std::convert::Into<u16> for $enum {
            fn into(self) -> u16 {
                match self {
                    $( $enum::$name => $id ,)*
                }
            }
        }
    };
}

pub use generate_id_table;
