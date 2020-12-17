// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Align;
use crate::Bin;
use crate::Buildable;
use crate::Container;
use crate::ResizeMode;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct Expander(Object<ffi::GtkExpander, ffi::GtkExpanderClass>) @extends Bin, Container, Widget, @implements Buildable;

    match fn {
        get_type => || ffi::gtk_expander_get_type(),
    }
}

impl Expander {
    #[doc(alias = "gtk_expander_new")]
    pub fn new(label: Option<&str>) -> Expander {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_expander_new(label.to_glib_none().0)).unsafe_cast()
        }
    }

    #[doc(alias = "gtk_expander_new_with_mnemonic")]
    pub fn with_mnemonic(label: &str) -> Expander {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_expander_new_with_mnemonic(label.to_glib_none().0))
                .unsafe_cast()
        }
    }
}

#[derive(Clone, Default)]
pub struct ExpanderBuilder {
    expanded: Option<bool>,
    label: Option<String>,
    label_fill: Option<bool>,
    label_widget: Option<Widget>,
    resize_toplevel: Option<bool>,
    spacing: Option<i32>,
    use_markup: Option<bool>,
    use_underline: Option<bool>,
    border_width: Option<u32>,
    child: Option<Widget>,
    resize_mode: Option<ResizeMode>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
}

impl ExpanderBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Expander {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref expanded) = self.expanded {
            properties.push(("expanded", expanded));
        }
        if let Some(ref label) = self.label {
            properties.push(("label", label));
        }
        if let Some(ref label_fill) = self.label_fill {
            properties.push(("label-fill", label_fill));
        }
        if let Some(ref label_widget) = self.label_widget {
            properties.push(("label-widget", label_widget));
        }
        if let Some(ref resize_toplevel) = self.resize_toplevel {
            properties.push(("resize-toplevel", resize_toplevel));
        }
        if let Some(ref spacing) = self.spacing {
            properties.push(("spacing", spacing));
        }
        if let Some(ref use_markup) = self.use_markup {
            properties.push(("use-markup", use_markup));
        }
        if let Some(ref use_underline) = self.use_underline {
            properties.push(("use-underline", use_underline));
        }
        if let Some(ref border_width) = self.border_width {
            properties.push(("border-width", border_width));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref resize_mode) = self.resize_mode {
            properties.push(("resize-mode", resize_mode));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        #[cfg(any(feature = "v3_20", feature = "dox"))]
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        let ret = glib::Object::new::<Expander>(&properties).expect("object new");
        ret
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = Some(expanded);
        self
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn label_fill(mut self, label_fill: bool) -> Self {
        self.label_fill = Some(label_fill);
        self
    }

    pub fn label_widget<P: IsA<Widget>>(mut self, label_widget: &P) -> Self {
        self.label_widget = Some(label_widget.clone().upcast());
        self
    }

    pub fn resize_toplevel(mut self, resize_toplevel: bool) -> Self {
        self.resize_toplevel = Some(resize_toplevel);
        self
    }

    pub fn spacing(mut self, spacing: i32) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn use_markup(mut self, use_markup: bool) -> Self {
        self.use_markup = Some(use_markup);
        self
    }

    pub fn use_underline(mut self, use_underline: bool) -> Self {
        self.use_underline = Some(use_underline);
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child<P: IsA<Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn resize_mode(mut self, resize_mode: ResizeMode) -> Self {
        self.resize_mode = Some(resize_mode);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent<P: IsA<Container>>(mut self, parent: &P) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }
}

pub const NONE_EXPANDER: Option<&Expander> = None;

pub trait ExpanderExt: 'static {
    #[doc(alias = "gtk_expander_get_expanded")]
    fn get_expanded(&self) -> bool;

    #[doc(alias = "gtk_expander_get_label")]
    fn get_label(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_expander_get_label_fill")]
    fn get_label_fill(&self) -> bool;

    #[doc(alias = "gtk_expander_get_label_widget")]
    fn get_label_widget(&self) -> Option<Widget>;

    #[doc(alias = "gtk_expander_get_resize_toplevel")]
    fn get_resize_toplevel(&self) -> bool;

    #[cfg_attr(feature = "v3_20", deprecated)]
    #[doc(alias = "gtk_expander_get_spacing")]
    fn get_spacing(&self) -> i32;

    #[doc(alias = "gtk_expander_get_use_markup")]
    fn get_use_markup(&self) -> bool;

    #[doc(alias = "gtk_expander_get_use_underline")]
    fn get_use_underline(&self) -> bool;

    #[doc(alias = "gtk_expander_set_expanded")]
    fn set_expanded(&self, expanded: bool);

    #[doc(alias = "gtk_expander_set_label")]
    fn set_label(&self, label: Option<&str>);

    #[doc(alias = "gtk_expander_set_label_fill")]
    fn set_label_fill(&self, label_fill: bool);

    #[doc(alias = "gtk_expander_set_label_widget")]
    fn set_label_widget<P: IsA<Widget>>(&self, label_widget: Option<&P>);

    #[doc(alias = "gtk_expander_set_resize_toplevel")]
    fn set_resize_toplevel(&self, resize_toplevel: bool);

    #[cfg_attr(feature = "v3_20", deprecated)]
    #[doc(alias = "gtk_expander_set_spacing")]
    fn set_spacing(&self, spacing: i32);

    #[doc(alias = "gtk_expander_set_use_markup")]
    fn set_use_markup(&self, use_markup: bool);

    #[doc(alias = "gtk_expander_set_use_underline")]
    fn set_use_underline(&self, use_underline: bool);

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_activate(&self);

    fn connect_property_expanded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_widget_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_resize_toplevel_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_20", deprecated)]
    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_underline_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Expander>> ExpanderExt for O {
    fn get_expanded(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_expander_get_expanded(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_label(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_expander_get_label(self.as_ref().to_glib_none().0)) }
    }

    fn get_label_fill(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_expander_get_label_fill(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_label_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_expander_get_label_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_resize_toplevel(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_expander_get_resize_toplevel(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_spacing(&self) -> i32 {
        unsafe { ffi::gtk_expander_get_spacing(self.as_ref().to_glib_none().0) }
    }

    fn get_use_markup(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_expander_get_use_markup(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_expander_get_use_underline(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_expanded(&self, expanded: bool) {
        unsafe {
            ffi::gtk_expander_set_expanded(self.as_ref().to_glib_none().0, expanded.to_glib());
        }
    }

    fn set_label(&self, label: Option<&str>) {
        unsafe {
            ffi::gtk_expander_set_label(self.as_ref().to_glib_none().0, label.to_glib_none().0);
        }
    }

    fn set_label_fill(&self, label_fill: bool) {
        unsafe {
            ffi::gtk_expander_set_label_fill(self.as_ref().to_glib_none().0, label_fill.to_glib());
        }
    }

    fn set_label_widget<P: IsA<Widget>>(&self, label_widget: Option<&P>) {
        unsafe {
            ffi::gtk_expander_set_label_widget(
                self.as_ref().to_glib_none().0,
                label_widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_resize_toplevel(&self, resize_toplevel: bool) {
        unsafe {
            ffi::gtk_expander_set_resize_toplevel(
                self.as_ref().to_glib_none().0,
                resize_toplevel.to_glib(),
            );
        }
    }

    fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_expander_set_spacing(self.as_ref().to_glib_none().0, spacing);
        }
    }

    fn set_use_markup(&self, use_markup: bool) {
        unsafe {
            ffi::gtk_expander_set_use_markup(self.as_ref().to_glib_none().0, use_markup.to_glib());
        }
    }

    fn set_use_underline(&self, use_underline: bool) {
        unsafe {
            ffi::gtk_expander_set_use_underline(
                self.as_ref().to_glib_none().0,
                use_underline.to_glib(),
            );
        }
    }

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkExpander,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Expander>,
        {
            let f: &F = &*(f as *const F);
            f(&Expander::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    activate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_activate(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("activate", &[])
                .unwrap()
        };
    }

    fn connect_property_expanded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_expanded_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkExpander,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Expander>,
        {
            let f: &F = &*(f as *const F);
            f(&Expander::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::expanded\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_expanded_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_label_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkExpander,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Expander>,
        {
            let f: &F = &*(f as *const F);
            f(&Expander::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::label\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_label_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_label_fill_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_label_fill_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkExpander,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Expander>,
        {
            let f: &F = &*(f as *const F);
            f(&Expander::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::label-fill\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_label_fill_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_label_widget_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_label_widget_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkExpander,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Expander>,
        {
            let f: &F = &*(f as *const F);
            f(&Expander::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::label-widget\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_label_widget_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_resize_toplevel_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_resize_toplevel_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkExpander,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Expander>,
        {
            let f: &F = &*(f as *const F);
            f(&Expander::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::resize-toplevel\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_resize_toplevel_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_spacing_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkExpander,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Expander>,
        {
            let f: &F = &*(f as *const F);
            f(&Expander::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::spacing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_spacing_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_markup_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkExpander,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Expander>,
        {
            let f: &F = &*(f as *const F);
            f(&Expander::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-markup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_markup_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_use_underline_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_underline_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkExpander,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Expander>,
        {
            let f: &F = &*(f as *const F);
            f(&Expander::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-underline\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_underline_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Expander {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Expander")
    }
}
