// SPDX-FileCopyrightText: 2022 Profian Inc. <opensource@profian.com>
// SPDX-License-Identifier: Apache-2.0

use core::ops::{Deref, RangeFrom};

use super::{Error, Essence, Parameters, Parsed};

/// A Media Type
#[derive(Clone, Debug)]
pub struct MediaType<T> {
    essence: Essence<T>,
    parameters: RangeFrom<usize>,
}

impl<'a> MediaType<&'a str> {
    /// Creates a new [`MediaType`] instance from a `&str`.
    ///
    /// For a version of this function that can work on any string type, see [`MediaType::new()`].
    pub const fn new_const(string: &'a str) -> Result<Self, Error> {
        // Parse the essence.
        let (essence, params) = match Essence::parse(string) {
            Ok(x) => x,
            Err(e) => return Err(e),
        };

        // Ensure the end of the essence is valid.
        if params.start == string.len() {
            if essence.species.end != params.start {
                return Err(Error(essence.species.end));
            }
        } else if string.as_bytes()[params.start] != b';' {
            return Err(Error(params.start));
        }

        // Confirm the validity of the parameters.
        let mut index = params.start..;
        while let Some(result) = Parameters::parse_next(string, index) {
            match result {
                Ok(Parsed { idx, .. }) => index = idx,
                Err(e) => return Err(e),
            }
        }

        Ok(Self {
            essence,
            parameters: params,
        })
    }
}

impl<T: AsRef<str>> MediaType<T> {
    /// Creates a new [`MediaType`] instance.
    ///
    /// For a `const` version of this function, see [`MediaType::new_const()`].
    pub fn new(string: T) -> Result<Self, Error> {
        let MediaType {
            parameters,
            essence:
                Essence {
                    string: _,
                    genus,
                    species,
                    tree,
                    suffix,
                },
        } = MediaType::new_const(string.as_ref())?;

        Ok(Self {
            parameters,
            essence: Essence {
                string,
                genus,
                species,
                tree,
                suffix,
            },
        })
    }

    /// Gets the essence of a [`MediaType`].
    ///
    /// # Example
    ///
    /// The essence of `text/plain; charset=US-ASCII` is `text/plain`.
    pub fn essence(&self) -> &Essence<T> {
        &self.essence
    }

    /// Fetch an iterator over the parameters of the [`MediaType`].
    pub fn parameters(&self) -> Parameters<'_> {
        Parameters(self.as_ref(), self.parameters.clone())
    }
}

impl<T> Deref for MediaType<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.essence.deref()
    }
}

impl<T: AsRef<U>, U: ?Sized> AsRef<U> for MediaType<T> {
    fn as_ref(&self) -> &U {
        self.essence.as_ref()
    }
}

impl<T: AsRef<str>> Eq for MediaType<T> {}

impl<T: AsRef<str>, U: AsRef<str>> PartialEq<Essence<U>> for MediaType<T> {
    fn eq(&self, other: &Essence<U>) -> bool {
        &self.essence == other
    }
}

impl<T: AsRef<str>, U: AsRef<str>> PartialEq<MediaType<U>> for MediaType<T> {
    fn eq(&self, other: &MediaType<U>) -> bool {
        self.essence == other.essence && self.parameters().eq(other.parameters())
    }
}

impl<T: AsRef<str>> PartialEq<str> for MediaType<T> {
    fn eq(&self, other: &str) -> bool {
        if let Ok(other) = MediaType::new(other) {
            return self == &other;
        }

        false
    }
}

impl<T: AsRef<str>> PartialEq<&str> for MediaType<T> {
    fn eq(&self, other: &&str) -> bool {
        if let Ok(other) = MediaType::new(other) {
            return self == &other;
        }

        false
    }
}

impl<T: AsRef<str>> PartialEq<MediaType<T>> for str {
    fn eq(&self, other: &MediaType<T>) -> bool {
        if let Ok(this) = MediaType::new(self) {
            return &this == other;
        }

        false
    }
}

impl<T: AsRef<str>> PartialEq<MediaType<T>> for &str {
    fn eq(&self, other: &MediaType<T>) -> bool {
        if let Ok(this) = MediaType::new(self) {
            return &this == other;
        }

        false
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T: AsRef<str>> PartialEq<MediaType<T>> for alloc::string::String {
    fn eq(&self, other: &MediaType<T>) -> bool {
        if let Ok(this) = MediaType::new(self) {
            return &this == other;
        }

        false
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl core::str::FromStr for MediaType<alloc::string::String> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.into())
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<T: AsRef<str> + serde::Serialize> serde::Serialize for MediaType<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.essence.string.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de, T: AsRef<str> + serde::Deserialize<'de>> serde::Deserialize<'de> for MediaType<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::new(T::deserialize(deserializer)?)
            .map_err(|_| <D::Error as serde::de::Error>::custom("invalid media type"))
    }
}

#[cfg(test)]
mod test {
    use super::{super::Uncased, Essence, MediaType};

    use alloc::collections::vec_deque::VecDeque;

    #[rstest::rstest]
    #[case("text/plainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainpl")]
    #[case("texttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttex/plain")]
    #[case("APPLICATION/VND.JOSE+JSON  ;  CHARSET=UTF-8  ;  CHARSET=US-ASCII")]
    #[case("APPLICATION/JOSE+JSON  ;  CHARSET=UTF-8  ;  CHARSET=US-ASCII")]
    #[case("APPLICATION/VND.JOSE  ;  CHARSET=UTF-8  ;  CHARSET=US-ASCII")]
    #[case("APPLICATION/JOSE  ;  CHARSET=UTF-8  ;  CHARSET=US-ASCII")]
    #[case("APPLICATION/VND.JOSE+JSON  ;  CHARSET=\"UTF-8\"")]
    #[case("APPLICATION/JOSE+JSON  ;  CHARSET=\"UTF-8\"")]
    #[case("APPLICATION/VND.JOSE  ;  CHARSET=\"UTF-8\"")]
    #[case("APPLICATION/JOSE  ;  CHARSET=\"UTF-8\"")]
    #[case("APPLICATION/VND.JOSE+JSON  ;  CHARSET=UTF-8")]
    #[case("APPLICATION/JOSE+JSON  ;  CHARSET=UTF-8")]
    #[case("APPLICATION/VND.JOSE  ;  CHARSET=UTF-8")]
    #[case("APPLICATION/JOSE  ;  CHARSET=UTF-8")]
    #[case("APPLICATION/VND.JOSE+JSON")]
    #[case("APPLICATION/JOSE+JSON")]
    #[case("APPLICATION/VND.JOSE")]
    #[case("APPLICATION/JOSE")]
    fn parse(#[case] mime: &str) {
        let mut all: VecDeque<_> = mime.split(';').map(|x| x.trim()).collect();
        eprintln!("{:?}", all);
        let base = all.pop_front().unwrap();
        let lower = base.to_ascii_lowercase();
        eprintln!("{:?}", lower);
        let (genus, species) = lower.split_once('/').unwrap();
        let tree = species.split_once('.').map(|x| Uncased(x.0));
        let suffix = species.rsplit_once('+').map(|x| Uncased(x.1));

        let e = MediaType::new(mime).unwrap();
        assert_eq!(e.essence(), lower.as_str());
        assert_eq!(e.essence().genus(), genus);
        assert_eq!(e.essence().species(), species);
        assert_eq!(e.essence().tree(), tree);
        assert_eq!(e.essence().suffix(), suffix);

        assert_eq!(all.len(), e.parameters().count());
        for ((k, v), x) in e.parameters().zip(all.iter()) {
            let (key, val) = x.split_once('=').unwrap();
            let key = key.trim_start().to_ascii_lowercase();
            let val = if val.starts_with('"') {
                assert!(val.ends_with('"'));
                val.strip_prefix('"').unwrap().strip_suffix('"').unwrap()
            } else {
                val
            };

            eprintln!("key: {:?}; {:?}", k, key);
            eprintln!("val: {:?}; {:?}", v, val);

            assert_eq!(k, key);
            assert_eq!(v, val);
        }
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
    #[case("text/plain;", 11)]
    #[case("text/plain; charsetUTF-8", 24)]
    #[case("text/plain; charset =UTF-8", 19)]
    #[case("text/plain; charset= UTF-8", 20)]
    #[case("text/plain; charset==UTF-8", 20)]
    #[case("text/plain; charset=\"UTF-8", 26)]
    #[case("text/plain; charse٦=UTF-8", 18)]
    #[case("text/plain; charset=U٦F-8", 21)]
    #[case("texttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttexttext/plain", 127)]
    #[case("text/plainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainplainpla", 132)]
    fn parse_error(#[case] string: &str, #[case] byte: usize) {
        assert_eq!(byte, MediaType::new(string).unwrap_err().0);
    }

    #[test]
    fn equality() {
        assert_eq!(
            MediaType::new("text/plain").unwrap(),
            MediaType::new("text/plain").unwrap()
        );

        assert_ne!(
            MediaType::new("text/plain; charset=UTF-8").unwrap(),
            MediaType::new("text/plain").unwrap()
        );

        assert_eq!(
            MediaType::new("text/plain; charset=UTF-8").unwrap(),
            Essence::new("text/plain").unwrap()
        );

        assert_eq!(
            MediaType::new("TEXT/PLAIN; CHARSET=UTF-8").unwrap(),
            "text/plain  ;  charset=UTF-8"
        );

        assert_ne!(
            MediaType::new("TEXT/PLAIN; CHARSET=UTF-8").unwrap(),
            "text / plain  ;  charset=UTF-8"
        );
    }
}
