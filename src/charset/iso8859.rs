/// Western European
pub mod iso8859_1 {
    use crate::charset::{
        iso6937::{
            singlechar_encode,
            singlechar_decode,
        },
        data::iso8859::{
            DECODE_MAP_1,
            HI_MAP_1,
            ENCODE_MAP_1,
        },
    };

    #[inline]
    pub fn encode(src: &str, dst: &mut Vec<u8>) { singlechar_encode(src, dst, &HI_MAP_1, &ENCODE_MAP_1) }

    #[inline]
    pub fn decode(src: &[u8], dst: &mut String) { singlechar_decode(src, dst, &DECODE_MAP_1) }
}


/// Central European
pub mod iso8859_2 {
    use crate::charset::{
        iso6937::{
            singlechar_encode,
            singlechar_decode,
        },
        data::iso8859::{
            DECODE_MAP_2,
            HI_MAP_2,
            ENCODE_MAP_2,
        },
    };

    #[inline]
    pub fn encode(src: &str, dst: &mut Vec<u8>) { singlechar_encode(src, dst, &HI_MAP_2, &ENCODE_MAP_2) }

    #[inline]
    pub fn decode(src: &[u8], dst: &mut String) { singlechar_decode(src, dst, &DECODE_MAP_2) }
}


/// South European
pub mod iso8859_3 {
    use crate::charset::{
        iso6937::{
            singlechar_encode,
            singlechar_decode,
        },
        data::iso8859::{
            DECODE_MAP_3,
            HI_MAP_3,
            ENCODE_MAP_3,
        },
    };

    #[inline]
    pub fn encode(src: &str, dst: &mut Vec<u8>) { singlechar_encode(src, dst, &HI_MAP_3, &ENCODE_MAP_3) }

    #[inline]
    pub fn decode(src: &[u8], dst: &mut String) { singlechar_decode(src, dst, &DECODE_MAP_3) }
}


/// North European
pub mod iso8859_4 {
    use crate::charset::{
        iso6937::{
            singlechar_encode,
            singlechar_decode,
        },
        data::iso8859::{
            DECODE_MAP_4,
            HI_MAP_4,
            ENCODE_MAP_4,
        },
    };

    #[inline]
    pub fn encode(src: &str, dst: &mut Vec<u8>) { singlechar_encode(src, dst, &HI_MAP_4, &ENCODE_MAP_4) }

    #[inline]
    pub fn decode(src: &[u8], dst: &mut String) { singlechar_decode(src, dst, &DECODE_MAP_4) }
}


/// Cyrillic
pub mod iso8859_5 {
    use crate::charset::{
        iso6937::{
            singlechar_encode,
            singlechar_decode,
        },
        data::iso8859::{
            DECODE_MAP_5,
            HI_MAP_5,
            ENCODE_MAP_5,
        },
    };

    #[inline]
    pub fn encode(src: &str, dst: &mut Vec<u8>) { singlechar_encode(src, dst, &HI_MAP_5, &ENCODE_MAP_5) }

    #[inline]
    pub fn decode(src: &[u8], dst: &mut String) { singlechar_decode(src, dst, &DECODE_MAP_5) }
}


/// Arabic
pub mod iso8859_6 {
    use crate::charset::{
        iso6937::{
            singlechar_encode,
            singlechar_decode,
        },
        data::iso8859::{
            DECODE_MAP_6,
            HI_MAP_6,
            ENCODE_MAP_6,
        },
    };

    #[inline]
    pub fn encode(src: &str, dst: &mut Vec<u8>) { singlechar_encode(src, dst, &HI_MAP_6, &ENCODE_MAP_6) }

    #[inline]
    pub fn decode(src: &[u8], dst: &mut String) { singlechar_decode(src, dst, &DECODE_MAP_6) }
}


/// Greek
pub mod iso8859_7 {
    use crate::charset::{
        iso6937::{
            singlechar_encode,
            singlechar_decode,
        },
        data::iso8859::{
            DECODE_MAP_7,
            HI_MAP_7,
            ENCODE_MAP_7,
        },
    };

    #[inline]
    pub fn encode(src: &str, dst: &mut Vec<u8>) { singlechar_encode(src, dst, &HI_MAP_7, &ENCODE_MAP_7) }

    #[inline]
    pub fn decode(src: &[u8], dst: &mut String) { singlechar_decode(src, dst, &DECODE_MAP_7) }
}


/// Hebrew
pub mod iso8859_8 {
    use crate::charset::{
        iso6937::{
            singlechar_encode,
            singlechar_decode,
        },
        data::iso8859::{
            DECODE_MAP_8,
            HI_MAP_8,
            ENCODE_MAP_8,
        },
    };

    #[inline]
    pub fn encode(src: &str, dst: &mut Vec<u8>) { singlechar_encode(src, dst, &HI_MAP_8, &ENCODE_MAP_8) }

    #[inline]
    pub fn decode(src: &[u8], dst: &mut String) { singlechar_decode(src, dst, &DECODE_MAP_8) }
}


/// Turkish
pub mod iso8859_9 {
    use crate::charset::{
        iso6937::{
            singlechar_encode,
            singlechar_decode,
        },
        data::iso8859::{
            DECODE_MAP_9,
            HI_MAP_9,
            ENCODE_MAP_9,
        },
    };

    #[inline]
    pub fn encode(src: &str, dst: &mut Vec<u8>) { singlechar_encode(src, dst, &HI_MAP_9, &ENCODE_MAP_9) }

    #[inline]
    pub fn decode(src: &[u8], dst: &mut String) { singlechar_decode(src, dst, &DECODE_MAP_9) }
}


/// Nordic
pub mod iso8859_10 {
    use crate::charset::{
        iso6937::{
            singlechar_encode,
            singlechar_decode,
        },
        data::iso8859::{
            DECODE_MAP_10,
            HI_MAP_10,
            ENCODE_MAP_10,
        },
    };

    #[inline]
    pub fn encode(src: &str, dst: &mut Vec<u8>) { singlechar_encode(src, dst, &HI_MAP_10, &ENCODE_MAP_10) }

    #[inline]
    pub fn decode(src: &[u8], dst: &mut String) { singlechar_decode(src, dst, &DECODE_MAP_10) }
}


/// Thai
pub mod iso8859_11 {
    use crate::charset::{
        iso6937::{
            singlechar_encode,
            singlechar_decode,
        },
        data::iso8859::{
            DECODE_MAP_11,
            HI_MAP_11,
            ENCODE_MAP_11,
        },
    };

    #[inline]
    pub fn encode(src: &str, dst: &mut Vec<u8>) { singlechar_encode(src, dst, &HI_MAP_11, &ENCODE_MAP_11) }

    #[inline]
    pub fn decode(src: &[u8], dst: &mut String) { singlechar_decode(src, dst, &DECODE_MAP_11) }
}


/// Baltic Rim
pub mod iso8859_13 {
    use crate::charset::{
        iso6937::{
            singlechar_encode,
            singlechar_decode,
        },
        data::iso8859::{
            DECODE_MAP_13,
            HI_MAP_13,
            ENCODE_MAP_13,
        },
    };

    #[inline]
    pub fn encode(src: &str, dst: &mut Vec<u8>) { singlechar_encode(src, dst, &HI_MAP_13, &ENCODE_MAP_13) }

    #[inline]
    pub fn decode(src: &[u8], dst: &mut String) { singlechar_decode(src, dst, &DECODE_MAP_13) }
}


/// Celtic
pub mod iso8859_14 {
    use crate::charset::{
        iso6937::{
            singlechar_encode,
            singlechar_decode,
        },
        data::iso8859::{
            DECODE_MAP_14,
            HI_MAP_14,
            ENCODE_MAP_14,
        },
    };

    #[inline]
    pub fn encode(src: &str, dst: &mut Vec<u8>) { singlechar_encode(src, dst, &HI_MAP_14, &ENCODE_MAP_14) }

    #[inline]
    pub fn decode(src: &[u8], dst: &mut String) { singlechar_decode(src, dst, &DECODE_MAP_14) }
}


/// Western European
pub mod iso8859_15 {
    use crate::charset::{
        iso6937::{
            singlechar_encode,
            singlechar_decode,
        },
        data::iso8859::{
            DECODE_MAP_15,
            HI_MAP_15,
            ENCODE_MAP_15,
        },
    };

    #[inline]
    pub fn encode(src: &str, dst: &mut Vec<u8>) { singlechar_encode(src, dst, &HI_MAP_15, &ENCODE_MAP_15) }

    #[inline]
    pub fn decode(src: &[u8], dst: &mut String) { singlechar_decode(src, dst, &DECODE_MAP_15) }
}


/// South-Eastern European
pub mod iso8859_16 {
    use crate::charset::{
        iso6937::{
            singlechar_encode,
            singlechar_decode,
        },
        data::iso8859::{
            DECODE_MAP_16,
            HI_MAP_16,
            ENCODE_MAP_16,
        },
    };

    #[inline]
    pub fn encode(src: &str, dst: &mut Vec<u8>) { singlechar_encode(src, dst, &HI_MAP_16, &ENCODE_MAP_16) }

    #[inline]
    pub fn decode(src: &[u8], dst: &mut String) { singlechar_decode(src, dst, &DECODE_MAP_16) }
}
