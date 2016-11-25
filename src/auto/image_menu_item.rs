// This file was generated by gir (5e8c56e) from gir-files (71d73f0)
// DO NOT EDIT

use AccelGroup;
use Actionable;
use Bin;
use Container;
use MenuItem;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct ImageMenuItem(Object<ffi::GtkImageMenuItem>): MenuItem, Bin, Container, Widget, Actionable;

    match fn {
        get_type => || ffi::gtk_image_menu_item_get_type(),
    }
}

impl ImageMenuItem {
    pub fn new() -> ImageMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_menu_item_new()).downcast_unchecked()
        }
    }

    pub fn new_from_stock(stock_id: &str, accel_group: Option<&AccelGroup>) -> ImageMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_menu_item_new_from_stock(stock_id.to_glib_none().0, accel_group.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_label(label: &str) -> ImageMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_menu_item_new_with_label(label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic(label: &str) -> ImageMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_menu_item_new_with_mnemonic(label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn get_always_show_image(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_image_menu_item_get_always_show_image(self.to_glib_none().0))
        }
    }

    pub fn get_image(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_image_menu_item_get_image(self.to_glib_none().0))
        }
    }

    pub fn get_use_stock(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_image_menu_item_get_use_stock(self.to_glib_none().0))
        }
    }

    pub fn set_accel_group(&self, accel_group: &AccelGroup) {
        unsafe {
            ffi::gtk_image_menu_item_set_accel_group(self.to_glib_none().0, accel_group.to_glib_none().0);
        }
    }

    pub fn set_always_show_image(&self, always_show: bool) {
        unsafe {
            ffi::gtk_image_menu_item_set_always_show_image(self.to_glib_none().0, always_show.to_glib());
        }
    }

    pub fn set_image<T: IsA<Widget>>(&self, image: Option<&T>) {
        unsafe {
            ffi::gtk_image_menu_item_set_image(self.to_glib_none().0, image.to_glib_none().0);
        }
    }

    pub fn set_use_stock(&self, use_stock: bool) {
        unsafe {
            ffi::gtk_image_menu_item_set_use_stock(self.to_glib_none().0, use_stock.to_glib());
        }
    }
}
