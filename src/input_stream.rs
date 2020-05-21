use std::borrow::{Borrow, Cow};
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

pub struct InputStream<'a, Data: ?Sized> {
    name: String,
    data_raw: &'a Data,
    index: isize,
}

impl<'a, Data: ?Sized + InputData, T: From<&'a Data>> CharStream<T> for InputStream<'a, Data> {
    /// Returns text from underlying string for start..=stop range
    #[inline]
    fn get_text(&self, start: isize, stop: isize) -> T {
        let stop = (stop + 1) as usize;
        let start = start as usize;
        // let start_offset = self.data[start as usize].0 as usize;
        let slice = if stop < self.data_raw.len() {
            // let stop_offset = self.data[stop].0 as usize;
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

#[cfg(test)]
mod test {
    use crate::int_stream::{EOF, IntStream};

    use super::InputStream;

    #[test]
    fn test_str_input_stream() {
        let mut input = InputStream::new("V1は");
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
        assert_eq!(input.la(1), EOF);
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
