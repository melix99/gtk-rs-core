// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use std::cmp;
use std::fmt;
use std::hash;
use std::mem;
use std::ops;
use std::ptr;
use std::slice;
use std::str;

wrapper! {
    // rustdoc-stripper-ignore-next
    /// A mutable text buffer that grows automatically.
    #[doc(alias = "GString")]
    #[must_use = "The builder must be built to be used"]
    pub struct GStringBuilder(BoxedInline<ffi::GString>);

    match fn {
        copy => |ptr| ffi::g_string_new((*ptr).str),
        free => |ptr| ffi::g_string_free(ptr, ffi::GTRUE),
        init => |ptr| unsafe {
            let inner = ffi::GString {
                str: ffi::g_malloc(64) as *mut _,
                len: 0,
                allocated_len: 64,
            };
            ptr::write(inner.str, 0);

            *ptr = inner;
        },
        copy_into => |dest, src| {
            assert!((*src).allocated_len > (*src).len);
            let allocated_len = (*src).allocated_len;
            let inner = ffi::GString {
                str: ffi::g_malloc(allocated_len) as *mut _,
                len: 0,
                allocated_len,
            };
            // +1 to also copy the NUL-terminator
            ptr::copy_nonoverlapping((*src).str, inner.str, (*src).len + 1);
            *dest = inner;
        },
        clear => |ptr| {
            ffi::g_free((*ptr).str as *mut _);
        },
        type_ => || ffi::g_gstring_get_type(),
    }
}

unsafe impl Send for GStringBuilder {}
unsafe impl Sync for GStringBuilder {}

impl GStringBuilder {
    #[doc(alias = "g_string_new_len")]
    pub fn new<T: AsRef<str>>(data: T) -> GStringBuilder {
        let data = data.as_ref();
        assert!(data.len() < usize::MAX - 1);
        unsafe {
            let allocated_len = usize::next_power_of_two(std::cmp::max(data.len(), 64) + 1);
            assert_ne!(allocated_len, 0);

            let inner = ffi::GString {
                str: ffi::g_malloc(allocated_len) as *mut _,
                len: data.len(),
                allocated_len,
            };
            if data.is_empty() {
                ptr::write(inner.str, 0);
            } else {
                ptr::copy_nonoverlapping(data.as_ptr() as *const _, inner.str, data.len());
                ptr::write(inner.str.add(data.len()), 0);
            }
            Self { inner }
        }
    }

    #[doc(alias = "g_string_append")]
    #[doc(alias = "g_string_append_len")]
    pub fn append(&mut self, val: &str) {
        unsafe {
            ffi::g_string_append_len(
                self.to_glib_none_mut().0,
                val.as_ptr() as *const _,
                val.len() as isize,
            );
        }
    }

    #[doc(alias = "g_string_prepend")]
    #[doc(alias = "g_string_prepend_len")]
    pub fn prepend(&mut self, val: &str) {
        unsafe {
            ffi::g_string_prepend_len(
                self.to_glib_none_mut().0,
                val.as_ptr() as *const _,
                val.len() as isize,
            );
        }
    }

    #[doc(alias = "g_string_append_c")]
    #[doc(alias = "g_string_append_unichar")]
    pub fn append_c(&mut self, val: char) {
        unsafe {
            ffi::g_string_append_unichar(self.to_glib_none_mut().0, val.into_glib());
        }
    }

    #[doc(alias = "g_string_prepend_c")]
    #[doc(alias = "g_string_prepend_unichar")]
    pub fn prepend_c(&mut self, val: char) {
        unsafe {
            ffi::g_string_prepend_unichar(self.to_glib_none_mut().0, val.into_glib());
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns `&str` slice.
    pub fn as_str(&self) -> &str {
        unsafe {
            let ptr: *const u8 = self.inner.str as _;
            let len: usize = self.inner.len;
            if len == 0 {
                return "";
            }
            let slice = slice::from_raw_parts(ptr, len);
            std::str::from_utf8_unchecked(slice)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns `&str` slice.
    #[must_use = "String returned from the builder should probably be used"]
    pub fn into_string(self) -> crate::GString {
        unsafe {
            let s = mem::ManuallyDrop::new(self);
            crate::GString::from_ptr_and_len_unchecked(s.inner.str, s.inner.len)
        }
    }
}

impl Default for GStringBuilder {
    // rustdoc-stripper-ignore-next
    /// Creates a new empty string.
    fn default() -> Self {
        Self::new("")
    }
}

impl fmt::Debug for GStringBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Display for GStringBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PartialEq for GStringBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<str> for GStringBuilder {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<GStringBuilder> for str {
    fn eq(&self, other: &GStringBuilder) -> bool {
        self == other.as_str()
    }
}

impl Eq for GStringBuilder {}

impl cmp::PartialOrd for GStringBuilder {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::PartialOrd<str> for GStringBuilder {
    fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
        Some(self.as_str().cmp(other))
    }
}

impl cmp::PartialOrd<GStringBuilder> for str {
    fn partial_cmp(&self, other: &GStringBuilder) -> Option<cmp::Ordering> {
        Some(self.cmp(other.as_str()))
    }
}

impl cmp::Ord for GStringBuilder {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl hash::Hash for GStringBuilder {
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.as_str().hash(state)
    }
}

impl AsRef<[u8]> for GStringBuilder {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<str> for GStringBuilder {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl ops::Deref for GStringBuilder {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Write for GStringBuilder {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.append(s);
        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.append_c(c);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn append() {
        let mut s = crate::GStringBuilder::new("");
        assert_eq!(&*s, "");
        s.append("Hello");
        s.append(" ");
        s.append("there!");
        assert_eq!(&*s, "Hello there!");
        assert_eq!(s.into_string().as_str(), "Hello there!");
    }

    #[test]
    fn prepend() {
        let mut s = crate::GStringBuilder::new("456");
        assert_eq!(&*s, "456");
        s.prepend("123");
        assert_eq!(&*s, "123456");
    }

    #[test]
    fn default() {
        let s1: crate::GStringBuilder = Default::default();
        assert_eq!(&*s1, "");
    }

    #[test]
    fn display() {
        let s: crate::GStringBuilder = crate::GStringBuilder::new("This is a string.");
        assert_eq!(&format!("{s}"), "This is a string.");
    }

    #[test]
    fn eq() {
        let a1 = crate::GStringBuilder::new("a");
        let a2 = crate::GStringBuilder::new("a");
        let b = crate::GStringBuilder::new("b");
        assert_eq!(a1, a2);
        assert_ne!(a1, b);
        assert_ne!(a2, b);
    }

    #[test]
    fn write() {
        use std::fmt::Write;

        let mut s = crate::GStringBuilder::default();
        write!(&mut s, "bla bla {} bla", 123).unwrap();
        assert_eq!(&*s, "bla bla 123 bla");
    }
}
