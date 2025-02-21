// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{translate::*, TimeType};

crate::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TimeZone(Shared<ffi::GTimeZone>);

    match fn {
        ref => |ptr| ffi::g_time_zone_ref(ptr),
        unref => |ptr| ffi::g_time_zone_unref(ptr),
        type_ => || ffi::g_time_zone_get_type(),
    }
}

impl TimeZone {
    #[cfg_attr(feature = "v2_68", deprecated = "Since 2.68")]
    #[allow(deprecated)]
    #[doc(alias = "g_time_zone_new")]
    pub fn new(identifier: Option<&str>) -> TimeZone {
        unsafe { from_glib_full(ffi::g_time_zone_new(identifier.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2_68", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_68")))]
    #[doc(alias = "g_time_zone_new_identifier")]
    #[doc(alias = "new_identifier")]
    pub fn from_identifier(identifier: Option<&str>) -> Option<TimeZone> {
        unsafe { from_glib_full(ffi::g_time_zone_new_identifier(identifier.to_glib_none().0)) }
    }

    #[doc(alias = "g_time_zone_new_local")]
    #[doc(alias = "new_local")]
    pub fn local() -> TimeZone {
        unsafe { from_glib_full(ffi::g_time_zone_new_local()) }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    #[doc(alias = "g_time_zone_new_offset")]
    #[doc(alias = "new_offset")]
    pub fn from_offset(seconds: i32) -> TimeZone {
        unsafe { from_glib_full(ffi::g_time_zone_new_offset(seconds)) }
    }

    #[doc(alias = "g_time_zone_new_utc")]
    #[doc(alias = "new_utc")]
    pub fn utc() -> TimeZone {
        unsafe { from_glib_full(ffi::g_time_zone_new_utc()) }
    }

    #[doc(alias = "g_time_zone_find_interval")]
    pub fn find_interval(&self, type_: TimeType, time_: i64) -> i32 {
        unsafe { ffi::g_time_zone_find_interval(self.to_glib_none().0, type_.into_glib(), time_) }
    }

    #[doc(alias = "g_time_zone_get_abbreviation")]
    #[doc(alias = "get_abbreviation")]
    pub fn abbreviation(&self, interval: i32) -> crate::GString {
        unsafe {
            from_glib_none(ffi::g_time_zone_get_abbreviation(
                self.to_glib_none().0,
                interval,
            ))
        }
    }

    #[cfg(any(feature = "v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_58")))]
    #[doc(alias = "g_time_zone_get_identifier")]
    #[doc(alias = "get_identifier")]
    pub fn identifier(&self) -> crate::GString {
        unsafe { from_glib_none(ffi::g_time_zone_get_identifier(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_time_zone_get_offset")]
    #[doc(alias = "get_offset")]
    pub fn offset(&self, interval: i32) -> i32 {
        unsafe { ffi::g_time_zone_get_offset(self.to_glib_none().0, interval) }
    }

    #[doc(alias = "g_time_zone_is_dst")]
    pub fn is_dst(&self, interval: i32) -> bool {
        unsafe { from_glib(ffi::g_time_zone_is_dst(self.to_glib_none().0, interval)) }
    }
}

unsafe impl Send for TimeZone {}
unsafe impl Sync for TimeZone {}
