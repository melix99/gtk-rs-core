// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "PangoCairoFontMap")]
    pub struct FontMap(Interface<ffi::PangoCairoFontMap>) @requires pango::FontMap;

    match fn {
        type_ => || ffi::pango_cairo_font_map_get_type(),
    }
}

impl FontMap {
    pub const NONE: Option<&'static FontMap> = None;

    #[doc(alias = "pango_cairo_font_map_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> pango::FontMap {
        unsafe { from_glib_none(ffi::pango_cairo_font_map_get_default()) }
    }
}

pub trait FontMapExt: 'static {
    #[doc(alias = "pango_cairo_font_map_get_resolution")]
    #[doc(alias = "get_resolution")]
    fn resolution(&self) -> f64;

    #[doc(alias = "pango_cairo_font_map_set_resolution")]
    fn set_resolution(&self, dpi: f64);
}

impl<O: IsA<FontMap>> FontMapExt for O {
    fn resolution(&self) -> f64 {
        unsafe { ffi::pango_cairo_font_map_get_resolution(self.as_ref().to_glib_none().0) }
    }

    fn set_resolution(&self, dpi: f64) {
        unsafe {
            ffi::pango_cairo_font_map_set_resolution(self.as_ref().to_glib_none().0, dpi);
        }
    }
}

impl fmt::Display for FontMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FontMap")
    }
}
