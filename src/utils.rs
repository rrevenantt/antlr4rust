use std::borrow::Borrow;

pub fn escape_whitespaces(data: impl Borrow<str>, escape_spaces: bool) -> String {
    let data = data.borrow();
    let mut res = String::with_capacity(data.len());
    data.chars().for_each(|ch| match ch {
        ' ' if escape_spaces => res.extend("\u{00B7}".chars()),
        '\t' => res.extend("\\t".chars()),
        '\n' => res.extend("\\n".chars()),
        '\r' => res.extend("\\r".chars()),
        _ => res.push(ch)
    });
    res
}