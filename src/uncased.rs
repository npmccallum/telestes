// SPDX-FileCopyrightText: 2022 Profian Inc. <opensource@profian.com>
// SPDX-License-Identifier: Apache-2.0

use core::hash::Hash;
use core::ops::{Deref, DerefMut};

/// A wrapper for uncased ASCII comparisons
#[derive(Copy, Clone, Debug)]
pub struct Uncased<T>(pub T);

impl<T: AsRef<str>> Hash for Uncased<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        for c in self.0.as_ref().chars() {
            c.to_ascii_lowercase().hash(state);
        }
    }
}

impl<T: AsRef<str>> Eq for Uncased<T> {}

impl<T: AsRef<str>, U: AsRef<str>> PartialEq<U> for Uncased<T> {
    fn eq(&self, other: &U) -> bool {
        self.0.as_ref().eq_ignore_ascii_case(other.as_ref())
    }
}

impl<T: AsRef<str>> PartialEq<Uncased<T>> for str {
    fn eq(&self, other: &Uncased<T>) -> bool {
        self.eq_ignore_ascii_case(other.as_ref())
    }
}

impl<T: AsRef<str>> PartialEq<Uncased<T>> for &str {
    fn eq(&self, other: &Uncased<T>) -> bool {
        self.eq_ignore_ascii_case(other.as_ref())
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T: AsRef<str>> PartialEq<Uncased<T>> for alloc::string::String {
    fn eq(&self, other: &Uncased<T>) -> bool {
        self.eq_ignore_ascii_case(other.as_ref())
    }
}

impl<T> Deref for Uncased<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Uncased<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: AsRef<U>, U: ?Sized> AsRef<U> for Uncased<T> {
    fn as_ref(&self) -> &U {
        self.0.as_ref()
    }
}
