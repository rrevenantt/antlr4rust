use std::borrow::{Borrow, Cow};
use std::char::{decode_utf16, DecodeUtf16Error, REPLACEMENT_CHARACTER};
use std::cmp::min;
use std::convert::TryFrom;
use std::io::Read;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::{Index, Range, RangeFrom, RangeFull};
use std::result;
use std::slice::{from_raw_parts, SliceIndex};
use std::str::{CharIndices, Chars};

use crate::char_stream::{CharStream, InputData};
use crate::errors::ANTLRError;
use crate::int_stream::{IntStream, EOF};
use crate::interval_set::Interval;
use crate::token::Token;

/// Default rust target input stream.
///
/// Since Rust uses UTF-8 format which does not support indexing,
/// `InputStream<str>` has slightly different index behavior in compare to java runtime when there are
/// non-ASCII unicode characters.
/// If you need it to generate exactly the same indexes as Java runtime, you have to use `CodePoint8/16/32BitCharStream`,
/// which does not use rusts native `str` type,
pub struct InputStream<'a, Data: ?Sized> {
    name: String,
    data_raw: &'a Data,
    index: isize,
}

impl<'a, T: From<&'a str>> CharStream<T> for InputStream<'a, str> {
    /// Returns text from underlying string for start..=stop range
    /// panics if provided indexes are invalid
    #[inline]
    fn get_text(&self, start: isize, stop: isize) -> T { self.get_text_inner(start, stop).into() }
}

pub type ByteCharStream<'a> = InputStream<'a, [u8]>;
pub type CodePoint8BitCharStream<'a> = InputStream<'a, [u8]>;
pub type CodePoint16BitCharStream<'a> = InputStream<'a, [u16]>;
pub type CodePoint32BitCharStream<'a> = InputStream<'a, [u32]>;

impl<'a, T> CharStream<&'a [T]> for InputStream<'a, [T]>
where
    [T]: InputData,
{
    fn get_text(&self, a: isize, b: isize) -> &'a [T] { self.get_text_inner(a, b).into() }
}

impl<'a, T> CharStream<String> for InputStream<'a, [T]>
where
    [T]: InputData,
{
    fn get_text(&self, a: isize, b: isize) -> String { self.get_text_inner(a, b).to_display() }
}

impl<'a, 'b, T> CharStream<Cow<'b, str>> for InputStream<'a, [T]>
where
    [T]: InputData,
{
    fn get_text(&self, a: isize, b: isize) -> Cow<'b, str> {
        self.get_text_inner(a, b).to_display().into()
    }
}

impl<'a, Data> InputStream<'a, Data>
where
    Data: InputData + ?Sized,
{
    fn get_text_inner(&self, start: isize, stop: isize) -> &'a Data {
        // println!("get text {}..{} of {:?}",start,stop,self.data_raw.to_display());
        let start = start as usize;
        let stop = self.data_raw.offset(stop, 1).unwrap_or(stop) as usize;
        // println!("justed range {}..{} ",start,stop);
        // let start = self.data_raw.offset(0,start).unwrap() as usize;
        // let stop = self.data_raw.offset(0,stop + 1).unwrap() as usize;

        if stop < self.data_raw.len() {
            &self.data_raw[start..stop]
        } else {
            &self.data_raw[start..]
        }
    }

    pub fn new(data_raw: &'a Data) -> Self {
        // let data_raw = data_raw.as_ref();
        // let data = data_raw.to_indexed_vec();
        Self {
            name: "<empty>".to_string(),
            data_raw,
            index: 0,
            // phantom: Default::default(),
        }
    }

    #[inline]
    pub fn reset(&mut self) { self.index = 0 }

    #[inline]
    pub fn lt(&mut self, offset: isize) -> isize { self.la(offset) }
}

impl<'a, Data: InputData + ?Sized> IntStream for InputStream<'a, Data> {
    #[inline]
    fn consume(&mut self) -> result::Result<(), ANTLRError> {
        if let Some(index) = self.data_raw.offset(self.index, 1) {
            self.index = index;
            Ok(())
        } else {
            Err(ANTLRError::IllegalStateError("cannot consume EOF".into()))
        }
    }

    #[inline]
    fn la(&mut self, mut offset: isize) -> isize {
        if offset == 0 {
            panic!("should not be called with offset 0");
        }
        if offset < 0 {
            offset += 1; // e.g., translate LA(-1) to use offset i=0; then data[p+0-1]
        }

        self.data_raw
            .offset(self.index, offset - 1)
            .and_then(|index| self.data_raw.item(index))
            .unwrap_or(crate::int_stream::EOF)
    }

    #[inline]
    fn mark(&mut self) -> isize { -1 }

    #[inline]
    fn release(&mut self, _marker: isize) {}

    #[inline]
    fn index(&self) -> isize { self.index }

    #[inline]
    fn seek(&mut self, mut index: isize) { self.index = index }

    #[inline]
    fn size(&self) -> isize { self.data_raw.len() as isize }

    fn get_source_name(&self) -> String { self.name.clone() }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;

    use crate::char_stream::CharStream;
    use crate::int_stream::{IntStream, EOF};

    use super::InputStream;

    #[test]
    fn test_str_input_stream() {
        let mut input = InputStream::new("V1は3");
        let input = &mut input as &mut dyn CharStream<String>;
        assert_eq!(input.la(1), 'V' as isize);
        assert_eq!(input.index(), 0);
        input.consume();
        assert_eq!(input.la(1), '1' as isize);
        assert_eq!(input.la(-1), 'V' as isize);
        assert_eq!(input.index(), 1);
        input.consume();
        assert_eq!(input.la(1), 0x306F);
        assert_eq!(input.index(), 2);
        input.consume();
        assert_eq!(input.index(), 5);
        assert_eq!(input.la(-2), '1' as isize);
        assert_eq!(input.la(2), EOF);
        assert_eq!(input.get_text(1, 1).deref(), "1");
        assert_eq!(input.get_text(1, 2).deref(), "1は");
        assert_eq!(input.get_text(2, 2).deref(), "は");
        assert_eq!(input.get_text(2, 5).deref(), "は3");
        assert_eq!(input.get_text(5, 5).deref(), "3");
    }

    #[test]
    fn test_byte_input_stream() {
        let mut input = InputStream::new(&b"V\xaa\xbb"[..]);
        assert_eq!(input.la(1), 'V' as isize);
        input.seek(2);
        assert_eq!(input.la(1), 0xBB);
        assert_eq!(input.index(), 2);
        let mut input = InputStream::new("は".as_bytes());
        assert_eq!(input.la(1), 227);
    }
}
