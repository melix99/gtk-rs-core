// This file was generated by gir (5e8c56e) from gir-files (71d73f0)
// DO NOT EDIT

use Actionable;
use Bin;
use Button;
use CheckButton;
use Container;
use ToggleButton;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct RadioButton(Object<ffi::GtkRadioButton>): CheckButton, ToggleButton, Button, Bin, Container, Widget, Actionable;

    match fn {
        get_type => || ffi::gtk_radio_button_get_type(),
    }
}

impl RadioButton {
    pub fn new(group: &[RadioButton]) -> RadioButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new(group.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_from_widget(radio_group_member: Option<&RadioButton>) -> RadioButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new_from_widget(radio_group_member.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_label(group: &[RadioButton], label: &str) -> RadioButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new_with_label(group.to_glib_none().0, label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_label_from_widget(radio_group_member: Option<&RadioButton>, label: &str) -> RadioButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new_with_label_from_widget(radio_group_member.to_glib_none().0, label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic(group: &[RadioButton], label: &str) -> RadioButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new_with_mnemonic(group.to_glib_none().0, label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic_from_widget(radio_group_member: Option<&RadioButton>, label: &str) -> RadioButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_button_new_with_mnemonic_from_widget(radio_group_member.to_glib_none().0, label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn get_group(&self) -> Vec<RadioButton> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_radio_button_get_group(self.to_glib_none().0))
        }
    }

    pub fn join_group(&self, group_source: Option<&RadioButton>) {
        unsafe {
            ffi::gtk_radio_button_join_group(self.to_glib_none().0, group_source.to_glib_none().0);
        }
    }

    pub fn set_group(&self, group: &[RadioButton]) {
        unsafe {
            ffi::gtk_radio_button_set_group(self.to_glib_none().0, group.to_glib_none().0);
        }
    }

    pub fn connect_group_changed<F: Fn(&RadioButton) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&RadioButton) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "group-changed",
                transmute(group_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn group_changed_trampoline(this: *mut ffi::GtkRadioButton, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&RadioButton) + 'static> = transmute(f);
    f(&from_glib_none(this))
}
