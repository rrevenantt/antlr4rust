use std::borrow::{Borrow, Cow};
use std::char::REPLACEMENT_CHARACTER;
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::iter::from_fn;
use std::num::TryFromIntError;
use std::ops::{Index, Range, RangeFrom};

use crate::int_stream::IntStream;
use crate::interval_set::Interval;
use crate::token::Token;
use crate::utils::Sealed;

/// Provides underlying data for Tokens.
pub trait CharStream<Data>: IntStream {
    /// Returns underlying data piece, either slice or owned copy.
    fn get_text(&self, a: isize, b: isize) -> Data;
    fn get_text_from_interval(&self, i: &Interval) -> Data { self.get_text(i.a, i.b) }
}

/// Trait for input that can be accepted by `InputStream` to be able to provide lexer with data.
/// Is sealed for now just in case.
pub trait InputData:
    Index<Range<usize>, Output = Self> + Index<RangeFrom<usize>, Output = Self> + ToOwned + 'static
{
    // fn to_indexed_vec(&self) -> Vec<(u32, u32)>;

    fn offset(&self, index: isize, item_offset: isize) -> Option<isize>;

    fn item(&self, index: isize) -> Option<isize>;

    fn len(&self) -> usize;

    fn from_text(text: &str) -> Self::Owned;

    fn to_display(&self) -> String;
}

impl<T: Into<u32> + From<u8> + TryFrom<u32> + Copy + Debug + 'static> InputData for [T]
where
    <T as TryFrom<u32>>::Error: Debug,
{
    // fn to_indexed_vec(&self) -> Vec<(u32, u32)> {
    //     self.into_iter()
    //         .enumerate()
    //         .map(|(x, &y)| (x as u32, y.into()))
    //         .collect()
    // }

    #[inline]
    fn offset(&self, index: isize, item_offset: isize) -> Option<isize> {
        let mut new_index = index + item_offset;
        if new_index < 0 {
            return None; // invalid; no char before first char
        }
        if new_index > self.len() as isize {
            return None;
        }

        Some(new_index)
    }

    #[inline]
    fn item(&self, index: isize) -> Option<isize> {
        self.get(index as usize).map(|&it| it.into() as isize)
    }

    #[inline]
    fn len(&self) -> usize { self.len() }

    #[inline]
    fn from_text(text: &str) -> Self::Owned {
        text.chars()
            .map(|it| T::try_from(it as u32).unwrap())
            .collect()
    }

    #[inline]
    // default
    fn to_display(&self) -> String {
        self.iter()
            .map(|x| char::try_from((*x).into()).unwrap_or(REPLACEMENT_CHARACTER))
            .collect()
    }
}
//
// impl InputData for [u8] {
//     #[inline]
//     fn to_display(&self) -> String { String::from_utf8_lossy(self).into_owned() }
// }

// impl InputData for [u16] {
// }
//
// impl InputData for [u32] {
//     #[inline]
//     fn to_display(&self) -> String {
//         self.iter()
//             .map(|x| char::try_from(*x).unwrap_or(REPLACEMENT_CHARACTER))
//             .collect()
//     }
// }

impl InputData for str {
    // fn to_indexed_vec(&self) -> Vec<(u32, u32)> {
    //     self.char_indices()
    //         .map(|(i, ch)| (i as u32, ch as u32))
    //         .collect()
    // }

    #[inline]
    fn offset(&self, mut index: isize, mut item_offset: isize) -> Option<isize> {
        if item_offset == 0 {
            return Some(index);
        }
        let direction = item_offset.signum();

        while {
            index += direction;
            if index < 0 || index > self.len() as isize {
                return None;
            }
            if self.is_char_boundary(index as usize) {
                item_offset -= direction;
            }
            item_offset != 0
        } {}

        Some(index)
    }

    #[inline]
    fn item(&self, index: isize) -> Option<isize> {
        self.get(index as usize..)
            .and_then(|it| it.chars().next())
            .map(|it| it as isize)
    }

    #[inline]
    fn len(&self) -> usize { self.len() }

    fn from_text(text: &str) -> Self::Owned { text.to_owned() }

    // #[inline]
    // fn from_text(text: &str) -> Self::Owned { text.to_owned() }

    #[inline]
    fn to_display(&self) -> String { self.to_string() }
}
