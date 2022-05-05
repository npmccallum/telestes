// SPDX-FileCopyrightText: 2022 Profian Inc. <opensource@profian.com>
// SPDX-License-Identifier: Apache-2.0

use core::ops::{Range, RangeFrom};

use super::{Error, Parsed, Uncased};

enum State {
    // Next character MUST be ';'.
    Semicolon,

    // Consume optional whitespace characters.
    Whitespace,

    // Currently capturing a key.
    Key(usize),

    // Previous character was '='.
    Equals(Range<usize>),

    // An unquoted value.
    Value(Range<usize>, usize),

    // A quoted value.
    Quoted(Range<usize>, usize),

    // Consume optional whitespace characters.
    Done(Range<usize>, Range<usize>),
}

/// Character is a token character.
const fn is_tc(c: u8) -> bool {
    match c {
        b'!' | b'#' | b'$' | b'&' | b'-' | b'^' | b'_' | b'.' | b'+' => true,
        b'%' | b'\'' | b'*' | b'`' | b'|' | b'~' => true,
        _ => c.is_ascii_alphanumeric(),
    }
}

/// Character is a quotable character.
const fn is_qc(c: u8) -> bool {
    match c {
        b'"' | b'\\' => false,
        b'!'..=b'~' => true,
        _ => false,
    }
}

/// An iterator for Media Type parameters
pub struct Parameters<'a>(pub(crate) &'a str, pub(crate) RangeFrom<usize>);

impl<'a> Parameters<'a> {
    #[allow(clippy::panic)]
    pub(crate) const fn parse_next(
        params: &str,
        offset: RangeFrom<usize>,
    ) -> Option<Result<Parsed, Error>> {
        if offset.start > params.len() {
            panic!("out of bounds");
        }

        let mut s = State::Semicolon;
        let mut i = offset.start;
        while i < params.len() {
            let c = params.as_bytes()[i];

            match s {
                // Ensure we start with a semicolon.
                State::Semicolon if c == b';' => s = State::Whitespace,
                State::Semicolon => return Some(Err(Error(i))),

                // Consume leading whitespace before the key.
                State::Whitespace if c.is_ascii_whitespace() => (),
                State::Whitespace if is_tc(c) => s = State::Key(i),
                State::Whitespace => return Some(Err(Error(i))),

                // Only accept token characters. '=' transitions to the next state.
                State::Key(idx) if c == b'=' => s = State::Equals(idx..i),
                State::Key(..) if is_tc(c) => (),
                State::Key(..) => return Some(Err(Error(i))),

                // Determine if the value is quoted or unquoted.
                State::Equals(key) if c == b'"' => s = State::Quoted(key, i + 1),
                State::Equals(key) if is_tc(c) => s = State::Value(key, i),
                State::Equals(..) => return Some(Err(Error(i))),

                // An unquoted value continues until whitespace or ';'.
                State::Value(key, val) if c.is_ascii_whitespace() => s = State::Done(key, val..i),
                State::Value(..) if is_tc(c) => (),
                State::Value(..) if c == b';' => break,
                State::Value(..) => return Some(Err(Error(i))),

                // A quoted value has quotable characters until the end quote.
                State::Quoted(key, val) if c == b'"' => s = State::Done(key, val..i),
                State::Quoted(..) if is_qc(c) => (),
                State::Quoted(..) => return Some(Err(Error(i))),

                // Consume trailing whitespace until the ';'.
                State::Done(..) if c.is_ascii_whitespace() => (),
                State::Done(..) if c == b';' => break,
                State::Done(..) => return Some(Err(Error(i))),
            }

            i += 1;
        }

        match s {
            State::Semicolon => None,
            State::Whitespace => Some(Err(Error(i))),
            State::Key(..) => Some(Err(Error(i))),
            State::Equals(..) => Some(Err(Error(i))),
            State::Value(key, val) => Some(Ok(Parsed {
                key,
                val: val..i,
                idx: i..,
            })),
            State::Quoted(..) => Some(Err(Error(i))),
            State::Done(key, val) => Some(Ok(Parsed { key, val, idx: i.. })),
        }
    }

    /// Finds the first parameter with the given name.
    pub fn get<K>(mut self, key: K) -> Option<&'a str>
    where
        Uncased<&'a str>: PartialEq<K>,
    {
        self.find_map(|x| (x.0 == key).then(|| x.1))
    }
}

impl<'a> Iterator for Parameters<'a> {
    type Item = (Uncased<&'a str>, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let parsed = Self::parse_next(self.0, self.1.clone())?.unwrap();
        self.1 = parsed.idx;
        Some((Uncased(&self.0[parsed.key]), &self.0[parsed.val]))
    }
}

#[cfg(test)]
mod test {
    use crate::MediaType;

    #[test]
    fn get() {
        let mt = MediaType::new("text/plain; foo=0; bar=\"baz()\"; bat=1").unwrap();
        assert_eq!(mt.parameters().get("bar"), Some("baz()"));
    }
}
