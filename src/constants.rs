// Copyright 2013-2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;


pub const MIME_TYPE_JPEG: &str = "image/jpeg";
pub const MIME_TYPE_PNG: &str = "image/png";
pub const MIME_TYPE_JP2: &str = "image/jp2";
pub const MIME_TYPE_URI: &str = "text/x-uri";
pub const MIME_TYPE_UNIQUE_ID: &str = "application/x-cairo.uuid";
pub const MIME_TYPE_JBIG2: &str = "application/x-cairo.jbig2";
pub const MIME_TYPE_JBIG2_GLOBAL: &str = "application/x-cairo.jbig2-global";
pub const MIME_TYPE_JBIG2_GLOBAL_ID: &str = "application/x-cairo.jbig2-global-id";
pub const MIME_TYPE_CCITT_FAX: &str = "image/g3fax";
pub const MIME_TYPE_CCITT_FAX_PARAMS: &str = "application/x-cairo.ccitt.params";
pub const MIME_TYPE_EPS: &str = "application/postscript";
pub const MIME_TYPE_EPS_PARAMS: &str = "application/x-cairo.eps.params";

pub const PDF_OUTLINE_ROOT: i32 = 0;
pub const PDF_OUTLINE_OPEN: i32 = ffi::PDF_OUTLINE_FLAG_OPEN;
pub const PDF_OUTLINE_BOLD: i32 = ffi::PDF_OUTLINE_FLAG_BOLD;
pub const PDF_OUTLINE_ITALIC: i32 = ffi::PDF_OUTLINE_FLAG_ITALIC;
