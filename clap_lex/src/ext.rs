use std::ffi::OsStr;

pub trait OsStrExt: private::Sealed {
    /// Converts to a string slice.
    fn try_str(&self) -> Result<&str, std::str::Utf8Error>;
    /// Returns `true` if the given pattern matches a sub-slice of
    /// this string slice.
    ///
    /// Returns `false` if it does not.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clap_lex::OsStrExt as _;
    /// let bananas = std::ffi::OsStr::new("bananas");
    ///
    /// assert!(bananas.contains("nana"));
    /// assert!(!bananas.contains("apples"));
    /// ```
    fn contains(&self, needle: &str) -> bool;
    /// Returns the byte index of the first character of this string slice that
    /// matches the pattern.
    ///
    /// Returns [`None`] if the pattern doesn't match.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use clap_lex::OsStrExt as _;
    /// let s = std::ffi::OsStr::new("Löwe 老虎 Léopard Gepardi");
    ///
    /// assert_eq!(s.find("L"), Some(0));
    /// assert_eq!(s.find("é"), Some(14));
    /// assert_eq!(s.find("par"), Some(17));
    /// ```
    ///
    /// Not finding the pattern:
    ///
    /// ```rust
    /// use clap_lex::OsStrExt as _;
    /// let s = std::ffi::OsStr::new("Löwe 老虎 Léopard");
    ///
    /// assert_eq!(s.find("1"), None);
    /// ```
    fn find(&self, needle: &str) -> Option<usize>;
    /// Returns a string slice with the prefix removed.
    ///
    /// If the string starts with the pattern `prefix`, returns substring after the prefix, wrapped
    /// in `Some`.
    ///
    /// If the string does not start with `prefix`, returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use clap_lex::OsStrExt as _;
    /// assert_eq!(OsStr::new("foo:bar").strip_prefix("foo:"), Some(OsStr::new("bar")));
    /// assert_eq!(OsStr::new("foo:bar").strip_prefix("bar"), None);
    /// assert_eq!(OsStr::new("foofoo").strip_prefix("foo"), Some(OsStr::new("foo")));
    /// ```
    fn strip_prefix(&self, prefix: &str) -> Option<&OsStr>;
    /// Returns `true` if the given pattern matches a prefix of this
    /// string slice.
    ///
    /// Returns `false` if it does not.
    ///
    /// # Examples
    ///
    /// ```
    /// use clap_lex::OsStrExt as _;
    /// let bananas = std::ffi::OsStr::new("bananas");
    ///
    /// assert!(bananas.starts_with("bana"));
    /// assert!(!bananas.starts_with("nana"));
    /// ```
    fn starts_with(&self, prefix: &str) -> bool;
    /// An iterator over substrings of this string slice, separated by
    /// characters matched by a pattern.
    ///
    /// # Examples
    ///
    /// Simple patterns:
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use clap_lex::OsStrExt as _;
    /// let v: Vec<_> = OsStr::new("Mary had a little lamb").split(" ").collect();
    /// assert_eq!(v, [OsStr::new("Mary"), OsStr::new("had"), OsStr::new("a"), OsStr::new("little"), OsStr::new("lamb")]);
    ///
    /// let v: Vec<_> = OsStr::new("").split("X").collect();
    /// assert_eq!(v, [OsStr::new("")]);
    ///
    /// let v: Vec<_> = OsStr::new("lionXXtigerXleopard").split("X").collect();
    /// assert_eq!(v, [OsStr::new("lion"), OsStr::new(""), OsStr::new("tiger"), OsStr::new("leopard")]);
    ///
    /// let v: Vec<_> = OsStr::new("lion::tiger::leopard").split("::").collect();
    /// assert_eq!(v, [OsStr::new("lion"), OsStr::new("tiger"), OsStr::new("leopard")]);
    /// ```
    ///
    /// If a string contains multiple contiguous separators, you will end up
    /// with empty strings in the output:
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use clap_lex::OsStrExt as _;
    /// let x = OsStr::new("||||a||b|c");
    /// let d: Vec<_> = x.split("|").collect();
    ///
    /// assert_eq!(d, &[OsStr::new(""), OsStr::new(""), OsStr::new(""), OsStr::new(""), OsStr::new("a"), OsStr::new(""), OsStr::new("b"), OsStr::new("c")]);
    /// ```
    ///
    /// Contiguous separators are separated by the empty string.
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use clap_lex::OsStrExt as _;
    /// let x = OsStr::new("(///)");
    /// let d: Vec<_> = x.split("/").collect();
    ///
    /// assert_eq!(d, &[OsStr::new("("), OsStr::new(""), OsStr::new(""), OsStr::new(")")]);
    /// ```
    ///
    /// Separators at the start or end of a string are neighbored
    /// by empty strings.
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use clap_lex::OsStrExt as _;
    /// let d: Vec<_> = OsStr::new("010").split("0").collect();
    /// assert_eq!(d, &[OsStr::new(""), OsStr::new("1"), OsStr::new("")]);
    /// ```
    ///
    /// When the empty string is used as a separator, it panics
    ///
    /// ```should_panic
    /// use std::ffi::OsStr;
    /// use clap_lex::OsStrExt as _;
    /// let f: Vec<_> = OsStr::new("rust").split("").collect();
    /// assert_eq!(f, &[OsStr::new(""), OsStr::new("r"), OsStr::new("u"), OsStr::new("s"), OsStr::new("t"), OsStr::new("")]);
    /// ```
    ///
    /// Contiguous separators can lead to possibly surprising behavior
    /// when whitespace is used as the separator. This code is correct:
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use clap_lex::OsStrExt as _;
    /// let x = OsStr::new("    a  b c");
    /// let d: Vec<_> = x.split(" ").collect();
    ///
    /// assert_eq!(d, &[OsStr::new(""), OsStr::new(""), OsStr::new(""), OsStr::new(""), OsStr::new("a"), OsStr::new(""), OsStr::new("b"), OsStr::new("c")]);
    /// ```
    ///
    /// It does _not_ give you:
    ///
    /// ```,ignore
    /// assert_eq!(d, &[OsStr::new("a"), OsStr::new("b"), OsStr::new("c")]);
    /// ```
    ///
    /// Use [`split_whitespace`] for this behavior.
    ///
    /// [`split_whitespace`]: str::split_whitespace
    fn split<'s, 'n>(&'s self, needle: &'n str) -> Split<'s, 'n>;
    /// Divide one string slice into two at an index.
    ///
    /// The argument, `mid`, should be a byte offset from the start of the
    /// string. It must also be on the boundary of a UTF-8 code point.
    ///
    /// The two slices returned go from the start of the string slice to `mid`,
    /// and from `mid` to the end of the string slice.
    ///
    /// To get mutable string slices instead, see the [`split_at_mut`]
    /// method.
    ///
    /// [`split_at_mut`]: str::split_at_mut
    ///
    /// # Panics
    ///
    /// Panics if `mid` is not on a UTF-8 code point boundary, or if it is
    /// past the end of the last code point of the string slice.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use clap_lex::OsStrExt as _;
    /// let s = std::ffi::OsStr::new("Per Martin-Löf");
    ///
    /// let (first, last) = s.split_at(3);
    ///
    /// assert_eq!("Per", first);
    /// assert_eq!(" Martin-Löf", last);
    /// ```
    fn split_at(&self, index: usize) -> (&OsStr, &OsStr);
    /// Splits the string on the first occurrence of the specified delimiter and
    /// returns prefix before delimiter and suffix after delimiter.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use clap_lex::OsStrExt as _;
    /// assert_eq!(OsStr::new("cfg").split_once("="), None);
    /// assert_eq!(OsStr::new("cfg=").split_once("="), Some((OsStr::new("cfg"), OsStr::new(""))));
    /// assert_eq!(OsStr::new("cfg=foo").split_once("="), Some((OsStr::new("cfg"), OsStr::new("foo"))));
    /// assert_eq!(OsStr::new("cfg=foo=bar").split_once("="), Some((OsStr::new("cfg"), OsStr::new("foo=bar"))));
    /// ```
    fn split_once(&self, needle: &'_ str) -> Option<(&OsStr, &OsStr)>;
}

impl OsStrExt for OsStr {
    fn try_str(&self) -> Result<&str, std::str::Utf8Error> {
        let bytes = to_bytes(self);
        std::str::from_utf8(bytes)
    }

    fn contains(&self, needle: &str) -> bool {
        self.find(needle).is_some()
    }

    fn find(&self, needle: &str) -> Option<usize> {
        (0..=self.len().checked_sub(needle.len())?)
            .find(|&x| to_bytes(self)[x..].starts_with(needle.as_bytes()))
    }

    fn strip_prefix(&self, prefix: &str) -> Option<&OsStr> {
        to_bytes(self)
            .strip_prefix(prefix.as_bytes())
            .map(to_os_str)
    }
    fn starts_with(&self, prefix: &str) -> bool {
        to_bytes(self).starts_with(prefix.as_bytes())
    }

    fn split<'s, 'n>(&'s self, needle: &'n str) -> Split<'s, 'n> {
        assert_ne!(needle, "");
        Split {
            haystack: Some(self),
            needle,
        }
    }

    fn split_at(&self, index: usize) -> (&OsStr, &OsStr) {
        let (first, second) = to_bytes(self).split_at(index);
        (to_os_str(first), to_os_str(second))
    }

    fn split_once(&self, needle: &'_ str) -> Option<(&OsStr, &OsStr)> {
        let start = self.find(needle)?;
        let end = start + needle.len();
        let haystack = to_bytes(self);
        let first = &haystack[0..start];
        let second = &haystack[end..];
        Some((to_os_str(first), to_os_str(second)))
    }
}

mod private {
    pub trait Sealed {}

    impl Sealed for std::ffi::OsStr {}
}

/// Allow access to raw bytes
///
/// **Note:** the bytes only make sense when compared with ASCII or `&str`
///
/// **Note:** This must never be serialized as there is no guarantee at how invalid UTF-8 will be
/// encoded, even within the same version of this crate (since its dependent on rustc version)
fn to_bytes(s: &OsStr) -> &[u8] {
    // SAFETY:
    // - Lifetimes are the same
    // - Types are compatible (`OsStr` is a transparent wrapper for `[u8]`)
    // - The primary contract is that the encoding for invalid surrogate code points is not
    //   guaranteed which isn't a problem here
    //
    // There is a proposal to support this natively (https://github.com/rust-lang/rust/pull/95290)
    // but its in limbo
    unsafe { std::mem::transmute(s) }
}

/// Restore raw bytes as `OsStr`
fn to_os_str(s: &[u8]) -> &OsStr {
    // SAFETY:
    // - Lifetimes are the same
    // - Types are compatible (`OsStr` is a transparent wrapper for `[u8]`)
    // - The primary contract is that the encoding for invalid surrogate code points is not
    //   guaranteed which isn't a problem here
    //
    // There is a proposal to support this natively (https://github.com/rust-lang/rust/pull/95290)
    // but its in limbo
    unsafe { std::mem::transmute(s) }
}

pub struct Split<'s, 'n> {
    haystack: Option<&'s OsStr>,
    needle: &'n str,
}

impl<'s, 'n> Iterator for Split<'s, 'n> {
    type Item = &'s OsStr;

    fn next(&mut self) -> Option<Self::Item> {
        let haystack = self.haystack?;
        match haystack.split_once(self.needle) {
            Some((first, second)) => {
                if !haystack.is_empty() {
                    debug_assert_ne!(haystack, second);
                }
                self.haystack = Some(second);
                Some(first)
            }
            None => {
                self.haystack = None;
                Some(haystack)
            }
        }
    }
}
