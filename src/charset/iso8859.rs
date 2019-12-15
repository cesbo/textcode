macro_rules! iso8859 {
    ( $($name: ident, $decode_map: ident, $hi_map: ident, $encode_map: ident),* ) => {
        $(
            pub mod $name {
                use crate::charset::{
                    iso6937::{
                        singlechar_encode,
                        singlechar_decode,
                    },
                    data::iso8859::{
                        $decode_map,
                        $hi_map,
                        $encode_map,
                    },
                };

                #[inline]
                pub fn encode(src: &str, dst: &mut Vec<u8>) {
                    singlechar_encode(src, dst, &$hi_map, &$encode_map)
                }

                #[inline]
                pub fn decode(src: &[u8], dst: &mut String) {
                    singlechar_decode(src, dst, &$decode_map)
                }

                #[inline]
                pub fn bound(_src: &[u8], limit: usize) -> usize {
                    limit
                }
            }
        )*
    }
}


iso8859!(
    iso8859_1, DECODE_MAP_1, HI_MAP_1, ENCODE_MAP_1, // Western European
    iso8859_2, DECODE_MAP_2, HI_MAP_2, ENCODE_MAP_2, // Central European
    iso8859_3, DECODE_MAP_3, HI_MAP_3, ENCODE_MAP_3, // South European
    iso8859_4, DECODE_MAP_4, HI_MAP_4, ENCODE_MAP_4, // North European
    iso8859_5, DECODE_MAP_5, HI_MAP_5, ENCODE_MAP_5, // Cyrillic
    iso8859_6, DECODE_MAP_6, HI_MAP_6, ENCODE_MAP_6, // Arabic
    iso8859_7, DECODE_MAP_7, HI_MAP_7, ENCODE_MAP_7, // Greek
    iso8859_8, DECODE_MAP_8, HI_MAP_8, ENCODE_MAP_8, // Hebrew
    iso8859_9, DECODE_MAP_9, HI_MAP_9, ENCODE_MAP_9, // Turkish
    iso8859_10, DECODE_MAP_10, HI_MAP_10, ENCODE_MAP_10, // Nordic
    iso8859_11, DECODE_MAP_11, HI_MAP_11, ENCODE_MAP_11, // Thai
    iso8859_13, DECODE_MAP_13, HI_MAP_13, ENCODE_MAP_13, // Baltic Rim
    iso8859_14, DECODE_MAP_14, HI_MAP_14, ENCODE_MAP_14, // Celtic
    iso8859_15, DECODE_MAP_15, HI_MAP_15, ENCODE_MAP_15, // Western European
    iso8859_16, DECODE_MAP_16, HI_MAP_16, ENCODE_MAP_16 // South-Eastern European
);
