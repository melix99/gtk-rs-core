// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::EventController;
#[cfg(any(feature = "v3_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
use crate::Widget;
#[cfg(any(feature = "v3_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
use glib::object::Cast;
#[cfg(any(feature = "v3_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct EventControllerMotion(Object<ffi::GtkEventControllerMotion, ffi::GtkEventControllerMotionClass>) @extends EventController;

    match fn {
        get_type => || ffi::gtk_event_controller_motion_get_type(),
    }
}

impl EventControllerMotion {
    #[cfg(any(feature = "v3_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
    #[doc(alias = "gtk_event_controller_motion_new")]
    pub fn new<P: IsA<Widget>>(widget: &P) -> EventControllerMotion {
        skip_assert_initialized!();
        unsafe {
            EventController::from_glib_full(ffi::gtk_event_controller_motion_new(
                widget.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    pub fn connect_enter<F: Fn(&EventControllerMotion, f64, f64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn enter_trampoline<F: Fn(&EventControllerMotion, f64, f64) + 'static>(
            this: *mut ffi::GtkEventControllerMotion,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"enter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    enter_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_leave<F: Fn(&EventControllerMotion) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn leave_trampoline<F: Fn(&EventControllerMotion) + 'static>(
            this: *mut ffi::GtkEventControllerMotion,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"leave\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    leave_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_motion<F: Fn(&EventControllerMotion, f64, f64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn motion_trampoline<
            F: Fn(&EventControllerMotion, f64, f64) + 'static,
        >(
            this: *mut ffi::GtkEventControllerMotion,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"motion\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    motion_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for EventControllerMotion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EventControllerMotion")
    }
}
