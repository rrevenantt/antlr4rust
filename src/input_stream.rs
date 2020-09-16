use std::borrow::{Borrow, Cow};
use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
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
use crate::int_stream::IntStream;
use crate::interval_set::Interval;
use crate::token::Token;

/// Default rust target input stream.
///
/// Since Rust uses UTF-8 format which does not support.
/// has slightly different index behavior in compare to java runtime when there are
/// non-ASCII unicode characters.
pub struct InputStream<'a, Data: ?Sized> {
    name: String,
    data_raw: &'a Data,
    index: isize,
}

impl<'a, Data: ?Sized + InputData, T: From<&'a Data>> CharStream<T> for InputStream<'a, Data> {
    /// Returns text from underlying string for start..=stop range
    /// panics if provided indexes are invalid
    #[inline]
    fn get_text(&self, start: isize, stop: isize) -> T {
        // println!("get text {}..{} of {:?}",start,stop,self.data_raw.to_display());
        let start = start as usize;
        let stop = self.data_raw.offset(stop,1).unwrap() as usize;
        // println!("justed range {}..{} ",start,stop);
        // let start = self.data_raw.offset(0,start).unwrap() as usize;
        // let stop = self.data_raw.offset(0,stop + 1).unwrap() as usize;

        let slice = if stop < self.data_raw.len() {
            &self.data_raw[start..stop]
        } else {
            &self.data_raw[start..]
        };

        slice.into()
    }
}

impl<'a, Data> InputStream<'a, Data>
    where
        Data: InputData + ?Sized,
{
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
/// Only for full backwards compatibility with Java runtime.
///
/// Use it only if you **have to** have exactly the same index output as in Java runtime because
/// there is an overhead to create underlying `Vec<u16>`.
#[deprecated]
pub struct UTF16InputStream{
    name: String,
    data: Vec<u16>,
    index: usize
}

impl CharStream<String> for UTF16InputStream{
    fn get_text(&self, a: isize, b: isize) -> String {
        String::from_utf16_lossy(&self.data[a as usize..1 + b as usize])
    }
}

impl IntStream for UTF16InputStream{
    fn consume(&mut self) -> Result<(), ANTLRError> {
        self.index += 1;
        if self.index > self.data.len() {
            Err(ANTLRError::IllegalStateError("cannot consume EOF".into()))
        } else {
            Ok(())
        }
    }

    fn la(&mut self, mut i: isize) -> isize {
        if i == 0 { panic!("invalid parameter") }

        i =  i + (i < 0) as isize;

        self.data[self.index + i as usize] as isize
    }

    fn mark(&mut self) -> isize { -1 }

    fn release(&mut self, marker: isize) {}

    fn index(&self) -> isize {
        self.index as isize
    }

    fn seek(&mut self, index: isize) {
        self.index = index as usize
    }

    fn size(&self) -> isize {
        self.data.len() as isize
    }

    fn get_source_name(&self) -> String {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;

    use crate::char_stream::CharStream;
    use crate::int_stream::{EOF, IntStream};

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
        assert_eq!(input.get_text(1,1).deref(),"1");
        assert_eq!(input.get_text(1,2).deref(),"1は");
        assert_eq!(input.get_text(2,2).deref(),"は");
        assert_eq!(input.get_text(2,5).deref(),"は3");
        assert_eq!(input.get_text(5,5).deref(),"3");
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
