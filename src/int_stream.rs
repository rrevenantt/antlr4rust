use crate::errors::ANTLRError;

pub const EOF: isize = -1;

pub trait IntStream {
    //todo handle return error everywhere
    fn consume(&mut self) -> Result<(), ANTLRError>;
    fn la(&mut self, i: isize) -> isize;
    fn mark(&mut self) -> isize;
    fn release(&mut self, marker: isize);
    fn index(&self) -> isize;
    fn seek(&mut self, index: isize);
    fn size(&self) -> isize;
    fn get_source_name(&self) -> String;
}

pub struct IterWrapper<'a, T: IntStream>(pub &'a mut T);

impl<'a, T: IntStream> Iterator for IterWrapper<'a, T> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.0.la(1);
        self.0.consume();
        match result {
            EOF => None,
            x => Some(x)
        }
    }
}
