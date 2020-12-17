// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Buildable;
use crate::Widget;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct Misc(Object<ffi::GtkMisc, ffi::GtkMiscClass>) @extends Widget, @implements Buildable;

    match fn {
        get_type => || ffi::gtk_misc_get_type(),
    }
}

impl Misc {}

pub const NONE_MISC: Option<&Misc> = None;

impl fmt::Display for Misc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Misc")
    }
}
