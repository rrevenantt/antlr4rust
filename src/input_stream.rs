use std::cmp::min;
use std::convert::TryFrom;
use std::io::*;
use std::iter::FromIterator;
use std::result;

use crate::char_stream::CharStream;
use crate::errors::ANTLRError;
use crate::int_stream::IntStream;
use crate::interval_set::Interval;
use crate::token::Token;

pub struct InputStream {
    name: String,
    index: isize,
    data: Vec<isize>,
//    size: isize,
}

impl InputStream {
    pub fn new(data: String) -> InputStream {
        let data = data.chars().map(|ch| ch as isize).collect();
        InputStream {
            name: "<empty>".to_string(),
            index: 0,
            data,
//            size: len,
        }
    }

//    pub fn from_read<T:IntoIterator<Item=char>>(read: T) -> Result<InputStream> {
//        let mut data = String::new();
//        let mut reader = BufReader::new(read);
//        let _size = reader.read_to_string(&mut data)?;
//        Ok(InputStream::new(data))
//    }

    pub fn reset(&mut self) {
        self.index = 0
    }

    pub fn lt(&mut self, offset: isize) -> isize {
        self.la(offset)
    }
}

impl CharStream for InputStream {
    fn get_text(&self, _start: isize, _stop: isize) -> String {
        let stop = min(self.data.len(), (_stop + 1) as usize);
        String::from_iter(self.data[_start as usize..stop].iter().map(|x| char::try_from(*x as u32).unwrap()))
    }

    fn get_text_from_tokens(&self, _start: &dyn Token, _stop: &dyn Token) -> &str {
        unimplemented!()
    }

    fn get_text_from_interval(&self, i: &Interval) -> String {
        self.get_text(i.a, i.b)
    }
}

impl IntStream for InputStream {
    fn consume(&mut self) -> result::Result<(), ANTLRError> {
        if self.index >= self.size() {
            return Err(ANTLRError::IllegalStateError("cannot consume EOF".into()));
        }
        self.index += 1;
        Ok(())
    }

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
            //System.out.println("char LA("+i+")=EOF; p="+p);
            return crate::int_stream::EOF;
        }
        //System.out.println("char LA("+i+")="+(char)data[p+i-1]+"; p="+p);
        //System.out.println("LA("+i+"); p="+p+" n="+n+" data.length="+data.length);
        return self.data[(self.index + offset - 1) as usize] as isize;
    }

    fn mark(&mut self) -> isize {
        -1
    }

    fn release(&mut self, _marker: isize) {}

    fn index(&self) -> isize {
        self.index
    }

    fn seek(&mut self, mut index: isize) {
        if index <= self.index {
            self.index = index
        }

        index = index.min(self.size());
        while self.index < index {
            self.consume();
        }
    }

    fn size(&self) -> isize {
        self.data.len() as isize
    }

    fn get_source_name(&self) -> String {
        self.name.clone()
    }
}
