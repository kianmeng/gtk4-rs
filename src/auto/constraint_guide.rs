// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use ConstraintStrength;
use ConstraintTarget;

glib_wrapper! {
    pub struct ConstraintGuide(Object<gtk_sys::GtkConstraintGuide, gtk_sys::GtkConstraintGuideClass, ConstraintGuideClass>) @implements ConstraintTarget;

    match fn {
        get_type => || gtk_sys::gtk_constraint_guide_get_type(),
    }
}

impl ConstraintGuide {
    pub fn new() -> ConstraintGuide {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gtk_sys::gtk_constraint_guide_new()) }
    }
}

impl Default for ConstraintGuide {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_CONSTRAINT_GUIDE: Option<&ConstraintGuide> = None;

pub trait ConstraintGuideExt: 'static {
    fn get_max_size(&self, width: i32, height: i32);

    fn get_min_size(&self, width: i32, height: i32);

    fn get_name(&self) -> Option<GString>;

    fn get_nat_size(&self, width: i32, height: i32);

    fn get_strength(&self) -> ConstraintStrength;

    fn set_max_size(&self, width: i32, height: i32);

    fn set_min_size(&self, width: i32, height: i32);

    fn set_name(&self, name: Option<&str>);

    fn set_nat_size(&self, width: i32, height: i32);

    fn set_strength(&self, strength: ConstraintStrength);

    fn get_property_max_height(&self) -> i32;

    fn set_property_max_height(&self, max_height: i32);

    fn get_property_max_width(&self) -> i32;

    fn set_property_max_width(&self, max_width: i32);

    fn get_property_min_height(&self) -> i32;

    fn set_property_min_height(&self, min_height: i32);

    fn get_property_min_width(&self) -> i32;

    fn set_property_min_width(&self, min_width: i32);

    fn get_property_nat_height(&self) -> i32;

    fn set_property_nat_height(&self, nat_height: i32);

    fn get_property_nat_width(&self) -> i32;

    fn set_property_nat_width(&self, nat_width: i32);

    fn connect_property_max_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_max_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_min_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_min_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_nat_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_nat_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_strength_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ConstraintGuide>> ConstraintGuideExt for O {
    fn get_max_size(&self, width: i32, height: i32) {
        unsafe {
            gtk_sys::gtk_constraint_guide_get_max_size(
                self.as_ref().to_glib_none().0,
                width,
                height,
            );
        }
    }

    fn get_min_size(&self, width: i32, height: i32) {
        unsafe {
            gtk_sys::gtk_constraint_guide_get_min_size(
                self.as_ref().to_glib_none().0,
                width,
                height,
            );
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_constraint_guide_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_nat_size(&self, width: i32, height: i32) {
        unsafe {
            gtk_sys::gtk_constraint_guide_get_nat_size(
                self.as_ref().to_glib_none().0,
                width,
                height,
            );
        }
    }

    fn get_strength(&self) -> ConstraintStrength {
        unsafe {
            from_glib(gtk_sys::gtk_constraint_guide_get_strength(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_max_size(&self, width: i32, height: i32) {
        unsafe {
            gtk_sys::gtk_constraint_guide_set_max_size(
                self.as_ref().to_glib_none().0,
                width,
                height,
            );
        }
    }

    fn set_min_size(&self, width: i32, height: i32) {
        unsafe {
            gtk_sys::gtk_constraint_guide_set_min_size(
                self.as_ref().to_glib_none().0,
                width,
                height,
            );
        }
    }

    fn set_name(&self, name: Option<&str>) {
        unsafe {
            gtk_sys::gtk_constraint_guide_set_name(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    fn set_nat_size(&self, width: i32, height: i32) {
        unsafe {
            gtk_sys::gtk_constraint_guide_set_nat_size(
                self.as_ref().to_glib_none().0,
                width,
                height,
            );
        }
    }

    fn set_strength(&self, strength: ConstraintStrength) {
        unsafe {
            gtk_sys::gtk_constraint_guide_set_strength(
                self.as_ref().to_glib_none().0,
                strength.to_glib(),
            );
        }
    }

    fn get_property_max_height(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"max-height\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `max-height` getter")
                .unwrap()
        }
    }

    fn set_property_max_height(&self, max_height: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"max-height\0".as_ptr() as *const _,
                Value::from(&max_height).to_glib_none().0,
            );
        }
    }

    fn get_property_max_width(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"max-width\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `max-width` getter")
                .unwrap()
        }
    }

    fn set_property_max_width(&self, max_width: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"max-width\0".as_ptr() as *const _,
                Value::from(&max_width).to_glib_none().0,
            );
        }
    }

    fn get_property_min_height(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"min-height\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `min-height` getter")
                .unwrap()
        }
    }

    fn set_property_min_height(&self, min_height: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"min-height\0".as_ptr() as *const _,
                Value::from(&min_height).to_glib_none().0,
            );
        }
    }

    fn get_property_min_width(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"min-width\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `min-width` getter")
                .unwrap()
        }
    }

    fn set_property_min_width(&self, min_width: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"min-width\0".as_ptr() as *const _,
                Value::from(&min_width).to_glib_none().0,
            );
        }
    }

    fn get_property_nat_height(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"nat-height\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `nat-height` getter")
                .unwrap()
        }
    }

    fn set_property_nat_height(&self, nat_height: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"nat-height\0".as_ptr() as *const _,
                Value::from(&nat_height).to_glib_none().0,
            );
        }
    }

    fn get_property_nat_width(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"nat-width\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `nat-width` getter")
                .unwrap()
        }
    }

    fn set_property_nat_width(&self, nat_width: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"nat-width\0".as_ptr() as *const _,
                Value::from(&nat_width).to_glib_none().0,
            );
        }
    }

    fn connect_property_max_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_height_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkConstraintGuide,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ConstraintGuide>,
        {
            let f: &F = &*(f as *const F);
            f(&ConstraintGuide::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_max_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_width_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkConstraintGuide,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ConstraintGuide>,
        {
            let f: &F = &*(f as *const F);
            f(&ConstraintGuide::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_min_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_height_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkConstraintGuide,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ConstraintGuide>,
        {
            let f: &F = &*(f as *const F);
            f(&ConstraintGuide::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::min-height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_min_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_width_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkConstraintGuide,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ConstraintGuide>,
        {
            let f: &F = &*(f as *const F);
            f(&ConstraintGuide::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::min-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkConstraintGuide,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ConstraintGuide>,
        {
            let f: &F = &*(f as *const F);
            f(&ConstraintGuide::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_nat_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_nat_height_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkConstraintGuide,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ConstraintGuide>,
        {
            let f: &F = &*(f as *const F);
            f(&ConstraintGuide::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::nat-height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_nat_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_nat_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_nat_width_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkConstraintGuide,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ConstraintGuide>,
        {
            let f: &F = &*(f as *const F);
            f(&ConstraintGuide::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::nat-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_nat_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_strength_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_strength_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkConstraintGuide,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ConstraintGuide>,
        {
            let f: &F = &*(f as *const F);
            f(&ConstraintGuide::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::strength\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_strength_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ConstraintGuide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ConstraintGuide")
    }
}
