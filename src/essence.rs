// SPDX-FileCopyrightText: 2022 Profian Inc. <opensource@profian.com>
// SPDX-License-Identifier: Apache-2.0

use core::ops::{Deref, Range, RangeFrom};

use crate::MediaType;

use super::{Error, Uncased};

// Character is a restricted name character.
const fn is_rnc(c: u8) -> bool {
    match c {
        b'!' | b'#' | b'$' | b'&' | b'-' | b'^' | b'_' | b'.' | b'+' => true,
        _ => c.is_ascii_alphanumeric(),
    }
}

/// The essence of a [`MediaType`].
///
/// # Example
///
/// The essence of `text/plain; charset=US-ASCII` is `text/plain`.
#[derive(Clone, Debug)]
pub struct Essence<T> {
    pub(crate) string: T,

    pub(crate) genus: Range<usize>,
    pub(crate) species: Range<usize>,

    pub(crate) tree: Option<Range<usize>>,
    pub(crate) suffix: Option<Range<usize>>,
}

impl<'a> Essence<&'a str> {
    const MAX: usize = 127;

    #[allow(clippy::if_same_then_else)]
    pub(crate) const fn parse(string: &'a str) -> Result<(Self, RangeFrom<usize>), Error> {
        let mut genus: Option<Range<usize>> = None;
        let mut species: Option<Range<usize>> = None;
        let mut tree: Option<Range<usize>> = None;
        let mut suffix: Option<Range<usize>> = None;

        let mut i = 0;
        while i < string.len() {
            let c = string.as_bytes()[i];

            if let Some(genus) = genus.as_ref() {
                if let Some(species) = species.as_ref() {
                    suffix = match suffix {
                        Some(x) => Some(x.start..species.end),
                        None => None,
                    };

                    if c == b';' {
                        break;
                    } else if !c.is_ascii_whitespace() {
                        return Err(Error(i));
                    }
                } else if i - genus.end > Self::MAX {
                    return Err(Error(i));
                } else if i == genus.end + 1 && !c.is_ascii_alphanumeric() {
                    return Err(Error(i));
                } else if c.is_ascii_whitespace() {
                    species = Some(genus.end + 1..i);
                } else if c == b';' {
                    species = Some(genus.end + 1..i);
                    continue; // Don't increment i.
                } else if !is_rnc(c) {
                    return Err(Error(i));
                } else if c == b'.' && tree.is_none() {
                    tree = Some(genus.end + 1..i);
                } else if i == string.len() - 1 {
                    species = Some(genus.end + 1..i + 1);
                    if let Some(prev) = suffix {
                        suffix = Some(prev.start..i + 1);
                    }
                } else if c == b'+' {
                    suffix = Some(i + 1..i + 1);
                }
            } else if c == b'/' {
                genus = Some(0..i);
            } else if i == 0 && !c.is_ascii_alphanumeric() {
                return Err(Error(i));
            } else if i >= Self::MAX {
                return Err(Error(i));
            } else if !is_rnc(c) {
                return Err(Error(i));
            }

            i += 1;
        }

        if let Some(genus) = genus {
            if let Some(species) = species {
                let essence = Self {
                    string,
                    genus,
                    species,
                    tree,
                    suffix,
                };

                return Ok((essence, i..));
            }

            return Err(Error(i));
        }

        Err(Error(i))
    }

    /// Creates a new [`Essence`] instance from a `&str`.
    ///
    /// For a version of this function that can work on any string type, see [`Essence::new()`].
    pub const fn new_const(s: &'a str) -> Result<Self, Error> {
        match Self::parse(s) {
            Ok((essence, end)) => {
                if essence.species.end != end.start {
                    Err(Error(essence.species.end))
                } else if end.start != s.len() {
                    Err(Error(end.start))
                } else {
                    Ok(essence)
                }
            }

            Err(e) => Err(e),
        }
    }
}

impl<T: AsRef<str>> Essence<T> {
    /// Creates a new [`Essence`] instance.
    ///
    /// For a `const` version of this function, see [`Essence::new_const()`].
    pub fn new(string: T) -> Result<Self, Error> {
        let Essence {
            string: _,
            genus,
            species,
            tree,
            suffix,
        } = Essence::new_const(string.as_ref())?;

        Ok(Self {
            string,
            genus,
            species,
            tree,
            suffix,
        })
    }

    fn name(&self, range: Range<usize>) -> Uncased<&str> {
        Uncased(&self.string.as_ref()[range])
    }

    fn body(&self) -> Uncased<&str> {
        self.name(self.genus.start..self.species.end)
    }

    /// Gets the genus of an [`Essence`].
    ///
    /// # Example
    ///
    /// The genus of `text/vnd.plain+json; charset=US-ASCII` is `text`.
    pub fn genus(&self) -> Uncased<&str> {
        self.name(self.genus.clone())
    }

    /// Gets the species of an [`Essence`].
    ///
    /// # Example
    ///
    /// The species of `text/vnd.plain+json; charset=US-ASCII` is `vnd.plain+json`.
    pub fn species(&self) -> Uncased<&str> {
        self.name(self.species.clone())
    }

    /// Gets the tree of an [`Essence`].
    ///
    /// # Example
    ///
    /// The tree of `text/vnd.plain+json; charset=US-ASCII` is `vnd`.
    pub fn tree(&self) -> Option<Uncased<&str>> {
        self.tree.clone().map(|r| self.name(r))
    }

    /// Gets the suffix of an [`Essence`].
    ///
    /// # Example
    ///
    /// The suffix of `text/vnd.plain+json; charset=US-ASCII` is `json`.
    pub fn suffix(&self) -> Option<Uncased<&str>> {
        self.suffix.clone().map(|r| self.name(r))
    }
}

impl<T> Deref for Essence<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.string
    }
}

impl<T: AsRef<U>, U: ?Sized> AsRef<U> for Essence<T> {
    fn as_ref(&self) -> &U {
        self.string.as_ref()
    }
}

impl<T: AsRef<str>> Eq for Essence<T> {}

impl<T: AsRef<str>, U: AsRef<str>> PartialEq<Essence<U>> for Essence<T> {
    fn eq(&self, other: &Essence<U>) -> bool {
        self.body() == other.body()
    }
}

impl<T: AsRef<str>, U: AsRef<str>> PartialEq<MediaType<U>> for Essence<T> {
    fn eq(&self, other: &MediaType<U>) -> bool {
        self == other.essence()
    }
}

impl<T: AsRef<str>> PartialEq<str> for Essence<T> {
    fn eq(&self, other: &str) -> bool {
        self.body() == other
    }
}

impl<T: AsRef<str>> PartialEq<&str> for Essence<T> {
    fn eq(&self, other: &&str) -> bool {
        self.body() == other
    }
}

impl<T: AsRef<str>> PartialEq<Essence<T>> for str {
    fn eq(&self, other: &Essence<T>) -> bool {
        self == other.body()
    }
}

impl<T: AsRef<str>> PartialEq<Essence<T>> for &str {
    fn eq(&self, other: &Essence<T>) -> bool {
        *self == *other.body()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T: AsRef<str>> PartialEq<Essence<T>> for alloc::string::String {
    fn eq(&self, other: &Essence<T>) -> bool {
        *self == *other.body()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl core::str::FromStr for Essence<alloc::string::String> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.into())
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<T: serde::Serialize> serde::Serialize for Essence<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.string.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de, T: AsRef<str> + serde::Deserialize<'de>> serde::Deserialize<'de> for Essence<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::new(T::deserialize(deserializer)?)
            .map_err(|_| <D::Error as serde::de::Error>::custom("invalid media type"))
    }
}

#[cfg(test)]
mod test {
    use super::{Essence, Uncased};

    #[rstest::rstest]
    #[case("text/plainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainpl")]
    #[case("texttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttex/plain")]
    #[case("APPLICATION/VND.JOSE+JSON")]
    #[case("APPLICATION/JOSE+JSON")]
    #[case("APPLICATION/VND.JOSE")]
    #[case("APPLICATION/JOSE")]
    #[case("AUDIO/AMR-WB+")]
    fn parse(#[case] mime: &str) {
        let lower = mime.to_ascii_lowercase();
        let (genus, species) = lower.split_once('/').unwrap();
        let tree = species.split_once('.').map(|x| Uncased(x.0));
        let mut suffix = species.rsplit_once('+').map(|x| Uncased(x.1));
        if let Some(Uncased("")) = suffix {
            suffix = None;
        }

        let e = Essence::new(mime).unwrap();
        assert_eq!(e, lower.as_ref());
        assert_eq!(e.genus(), genus);
        assert_eq!(e.species(), species);
        assert_eq!(e.tree(), tree);
        assert_eq!(e.suffix(), suffix);
    }

    #[rstest::rstest]
    #[case("", 0)]
    #[case("textplain", 9)]
    #[case("text//plain", 5)]
    #[case(" text/plain", 0)]
    #[case("text /plain", 4)]
    #[case("text/ plain", 5)]
    #[case("t٤xt/plain", 1)]
    #[case("text/p١ain", 6)]
    #[case("text/plain ", 10)]
    #[case("text/plain;", 10)]
    #[case("text/plain; charset=UTF-8", 10)]
    #[case("texttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttext/plain", 127)]
    #[case("text/plainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainpla", 132)]
    fn parse_fail(#[case] string: &str, #[case] byte: usize) {
        assert_eq!(byte, Essence::new(string).unwrap_err().0);
    }
}
