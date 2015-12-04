use encoding::types::{ DecoderTrap, EncodingRef };
use encoding::all::encodings;

use std::ops::{ Range, Deref, Index };
use std::borrow::Borrow;
use std::fmt;

// shorthand-exports for construction
pub use self::Text::Raw as tr;
pub use self::Text::Utf8 as tu;
pub use self::TextSlice::Raw as tsr;
pub use self::TextSlice::Utf8 as tsu;

/// Safe wrapper around something that's supposed to represent text.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Text {
    Raw(Vec<u8>),
    Utf8(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TextSlice<'a> {
    Raw(&'a [u8]),
    Utf8(&'a str)
}

pub const EMPTY_RAW: TextSlice<'static> = TextSlice::Raw(&[]);
pub const EMPTY_UTF8: TextSlice<'static> = TextSlice::Utf8("");

impl Text {
    pub fn decode_with(&self, e: EncodingRef, d: DecoderTrap) -> Text {
        match self {
            &Text::Raw(b) => match e.decode(&b, d) {
                Ok(s) => Text::Utf8(s),
                Err(_) => Text::Raw(b)
            },
            &Text::Utf8(s) => Text::Utf8(s)
        }
    }

    pub fn try_decode_with(&self, e: EncodingRef) -> Text {
        self.decode_with(DecoderTrap::Strict)
    }

    pub fn try_decode_as(&self, e: &str) -> Option<Text> {
        encoding(e).map(|e| self.try_decode_with(e))
    }

    pub fn lossy_decode_with(&self, e: EncodingRef) -> Text {
        self.decode_with(DecoderTrap::Replace)
    }

    pub fn lossy_decode_as(&self, e: &str) -> Option<Text> {
        encoding(e).map(|e| self.lossy_decode_with(e))
    }

    pub fn raw(&self) -> Option<&[u8]> {
        match self {
            &Text::Raw(ref b) => Some(b),
            _ => None
        }
    }

    pub fn utf8(&self) -> Option<&str> {
        match self {
            &Text::Utf8(ref s) => Some(s),
            _ => None
        }
    }

    pub fn slice<'a>(&'a self, r: &Range<usize>) -> TextSlice<'a> {
        match self {
            &Text::Raw(ref b) => TextSlice::Raw(&b[r]),
            &Text::Utf8(ref s) => TextSlice::Utf8(&s[r])
        }
    }

    pub fn length(&self) -> usize {
        match self {
            &Text::Raw(ref b) => b.len(),
            &Text::Utf8(ref s) => s.len()
        }
    }
}

impl<'a> TextSlice<'a> {
    pub fn raw(&self) -> Option<&[u8]> {
        match self {
            &TextSlice::Raw(ref b) => Some(b),
            _ => None
        }
    }

    pub fn utf8(&self) -> Option<&str> {
        match self {
            &TextSlice::Utf8(ref s) => Some(s),
            _ => None
        }
    }
}

impl<'a> From<TextSlice<'a>> for Text {
    fn from(ts: TextSlice<'a>) -> Text {
        match ts {
            TextSlice::Raw(b) => Text::Raw(b.into()),
            TextSlice::Utf8(s) => Text::Utf8(s.into())
        }
    }
}

impl<'a> From<&'a Text> for TextSlice<'a> {
    fn from(t: &'a Text) -> TextSlice<'a> {
        match t {
            &Text::Raw(ref b) => TextSlice::Raw(b),
            &Text::Utf8(ref s) => TextSlice::Utf8(s)
        }
    }
}

impl<'a> From<&'a str> for Text {
    fn from(s: &'a str) -> Text {
        Text::Utf8(s.into())
    }
}

impl<'a> From<&'a [u8]> for Text {
    fn from(b: &'a [u8]) -> Text {
        Text::Raw(b.into())
    }
}

impl<'a> From<&'a str> for TextSlice<'a> {
    fn from(s: &'a str) -> TextSlice<'a> {
        TextSlice::Utf8(s)
    }
}

impl<'a> From<&'a [u8]> for TextSlice<'a> {
    fn from(b: &'a [u8]) -> TextSlice<'a> {
        TextSlice::Raw(b)
    }
}


impl<'a> Deref for TextSlice<'a> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        match self {
            &Text::Raw(ref b) => b,
            &Text::Utf8(ref s) => s.as_bytes()
        }
    }
}

impl Deref for Text {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        match self {
            &Text::Raw(ref b) => b,
            &Text::Utf8(ref s) => s.as_bytes()
        }
    }
}

/*impl<'a> PartialEq for TextSlice<'a> {
    fn eq(&self, rhs: &TextSlice<'a>) -> bool {
        match (self, rhs) {
            (&tsr(ref b), &tsr(ref c)) => b == c,
            (&tsu(ref b), &tsu(ref c)) => b == c,
            _ => false
        }
    }
}

impl PartialEq for Text {
    fn eq(&self, rhs: &Text) -> bool {
        match (self, rhs) {
            (&tr(ref b), &tr(ref c)) => b == c,
            (&tu(ref b), &tu(ref c)) => b == c,
            _ => false
        }
    }
}*/

pub fn encoding(s: &str) -> Option<EncodingRef> {
    encodings().into_iter().cloned().find(|e| e.name() == s)
}

pub fn lossy_decode(b: &[u8], e: EncodingRef) -> String {
    e.decode(b, DecoderTrap::Replace)
        .ok().expect("Shouldn't error with replacing trap")
}

pub fn def_lossy_decode(b: &[u8]) -> String {
    lossy_decode(b, ::ENCODING)
}
