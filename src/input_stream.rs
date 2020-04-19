use std::borrow::{Borrow, Cow};
use std::cmp::min;
use std::convert::TryFrom;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::result;
use std::slice::from_raw_parts;
use std::str::Chars;

use crate::char_stream::CharStream;
use crate::errors::ANTLRError;
use crate::int_stream::IntStream;
use crate::interval_set::Interval;
use crate::token::Token;

pub struct InputStream<'a, T: 'a + From<&'a str>> {
    name: String,
    data_raw: Vec<&'a str>,
    index: isize,
    iter: Chars<'a>,
    data: Vec<isize>,
    phantom: PhantomData<fn() -> T>,
}

impl<'a> InputStream<'a, &'a str> {
    pub fn new(data_raw: &'a str) -> Self {
        Self::new_inner(data_raw)
    }
}

impl<'a> CharStream<'a> for InputStream<'a, &'a str> {
    type T = &'a str;

    fn get_text(&self, a: isize, b: isize) -> &'a str {
        self.text(a, b)
    }
}

impl<'a> InputStream<'a, String> {
    pub fn owned_stream(data_raw: &'a str) -> Self {
        Self::new_inner(data_raw)
    }
}

impl<'a> CharStream<'_> for InputStream<'a, String> {
    type T = String;

    fn get_text(&self, a: isize, b: isize) -> String {
        self.text(a, b).into()
    }
}

impl<'a, T: 'a + From<&'a str>> InputStream<'a, T> {
    fn new_inner(data_raw: &'a str) -> Self {
        let data = data_raw.chars().map(|ch| ch as isize).collect();
        Self {
            name: "<empty>".to_string(),
            data_raw: vec![data_raw],
            index: 0,
            iter: data_raw.chars(),
            data,
            phantom: Default::default(),
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        self.index = 0
    }

    #[inline]
    pub fn lt(&mut self, offset: isize) -> isize {
        self.la(offset)
    }

    /// Returns text from underlying string for start..=stop range
    #[inline]
    fn text(&self, start: isize, stop: isize) -> &'a str {
        if ((stop + 1) as usize) < self.data_raw.len() {
            let stop_offset = self.data_raw[(stop + 1) as usize].as_ptr() as usize - self.data_raw[start as usize].as_ptr() as usize;
            &self.data_raw[start as usize][..stop_offset]
        } else {
            self.data_raw[start as usize]
        }
    }
}

impl<'a, T: 'a + From<&'a str>> IntStream for InputStream<'a, T> {
    #[inline]
    fn consume(&mut self) -> result::Result<(), ANTLRError> {
        if self.index >= self.size() {
            return Err(ANTLRError::IllegalStateError("cannot consume EOF".into()));
        }
        self.index += 1;
        if self.data_raw.len() >= self.index as usize {
            self.iter.next();
            self.data_raw.push(self.iter.as_str())
        }
        Ok(())
    }

    #[inline]
    fn la(&mut self, mut offset: isize) -> isize {
        if offset == 0 {
            return 0;
        }
        if offset < 0 {
            offset += 1; // e.g., translate LA(-1) to use offset i=0; then data[p+0-1]
            if (self.index + offset - 1) < 0 {
                return crate::int_stream::EOF; // invalid; no char before first char
            }
        }

        if (self.index + offset - 1) >= self.size() {
            return crate::int_stream::EOF;
        }
        return self.data[(self.index + offset - 1) as usize] as isize;
    }

    #[inline]
    fn mark(&mut self) -> isize {
        -1
    }

    #[inline]
    fn release(&mut self, _marker: isize) {}

    #[inline]
    fn index(&self) -> isize {
        self.index
    }

    #[inline]
    fn seek(&mut self, mut index: isize) {
        if index <= self.index {
            self.index = index
        }

        index = index.min(self.size());
        while self.index < index {
            self.consume();
        }
    }

    #[inline]
    fn size(&self) -> isize {
        self.data.len() as isize
    }

    fn get_source_name(&self) -> String {
        self.name.clone()
    }
}

