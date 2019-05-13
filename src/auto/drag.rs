// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ContentFormats;
use ContentProvider;
use Device;
use Display;
use DragAction;
use DragCancelReason;
use Surface;
use ffi;
use glib::StaticType;
use glib::Value;
use glib::object::IsA;
use glib::object::ObjectType;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Drag(Object<ffi::GdkDrag, DragClass>);

    match fn {
        get_type => || ffi::gdk_drag_get_type(),
    }
}

impl Drag {
    pub fn drop_done(&self, success: bool) {
        unsafe {
            ffi::gdk_drag_drop_done(self.to_glib_none().0, success.to_glib());
        }
    }

    pub fn get_actions(&self) -> DragAction {
        unsafe {
            from_glib(ffi::gdk_drag_get_actions(self.to_glib_none().0))
        }
    }

    pub fn get_device(&self) -> Option<Device> {
        unsafe {
            from_glib_none(ffi::gdk_drag_get_device(self.to_glib_none().0))
        }
    }

    pub fn get_display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_drag_get_display(self.to_glib_none().0))
        }
    }

    pub fn get_drag_surface(&self) -> Option<Surface> {
        unsafe {
            from_glib_none(ffi::gdk_drag_get_drag_surface(self.to_glib_none().0))
        }
    }

    pub fn get_formats(&self) -> Option<ContentFormats> {
        unsafe {
            from_glib_none(ffi::gdk_drag_get_formats(self.to_glib_none().0))
        }
    }

    pub fn get_selected_action(&self) -> DragAction {
        unsafe {
            from_glib(ffi::gdk_drag_get_selected_action(self.to_glib_none().0))
        }
    }

    pub fn set_hotspot(&self, hot_x: i32, hot_y: i32) {
        unsafe {
            ffi::gdk_drag_set_hotspot(self.to_glib_none().0, hot_x, hot_y);
        }
    }

    pub fn set_property_actions(&self, actions: DragAction) {
        unsafe {
            gobject_ffi::g_object_set_property(self.as_ptr() as *mut gobject_ffi::GObject, b"actions\0".as_ptr() as *const _, Value::from(&actions).to_glib_none().0);
        }
    }

    pub fn get_property_content(&self) -> Option<ContentProvider> {
        unsafe {
            let mut value = Value::from_type(<ContentProvider as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.as_ptr() as *mut gobject_ffi::GObject, b"content\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    pub fn set_property_selected_action(&self, selected_action: DragAction) {
        unsafe {
            gobject_ffi::g_object_set_property(self.as_ptr() as *mut gobject_ffi::GObject, b"selected-action\0".as_ptr() as *const _, Value::from(&selected_action).to_glib_none().0);
        }
    }

    pub fn get_property_surface(&self) -> Option<Surface> {
        unsafe {
            let mut value = Value::from_type(<Surface as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.as_ptr() as *mut gobject_ffi::GObject, b"surface\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    pub fn begin<P: IsA<Surface>, Q: IsA<ContentProvider>>(surface: &P, device: &Device, content: &Q, actions: DragAction, dx: i32, dy: i32) -> Option<Drag> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gdk_drag_begin(surface.as_ref().to_glib_none().0, device.to_glib_none().0, content.as_ref().to_glib_none().0, actions.to_glib(), dx, dy))
        }
    }

    pub fn connect_cancel<F: Fn(&Drag, DragCancelReason) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"cancel\0".as_ptr() as *const _,
                Some(transmute(cancel_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_dnd_finished<F: Fn(&Drag) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"dnd-finished\0".as_ptr() as *const _,
                Some(transmute(dnd_finished_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_drop_performed<F: Fn(&Drag) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"drop-performed\0".as_ptr() as *const _,
                Some(transmute(drop_performed_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_actions_notify<F: Fn(&Drag) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::actions\0".as_ptr() as *const _,
                Some(transmute(notify_actions_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_display_notify<F: Fn(&Drag) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::display\0".as_ptr() as *const _,
                Some(transmute(notify_display_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_selected_action_notify<F: Fn(&Drag) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::selected-action\0".as_ptr() as *const _,
                Some(transmute(notify_selected_action_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn cancel_trampoline<F: Fn(&Drag, DragCancelReason) + 'static>(this: *mut ffi::GdkDrag, reason: ffi::GdkDragCancelReason, f: glib_ffi::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this), from_glib(reason))
}

unsafe extern "C" fn dnd_finished_trampoline<F: Fn(&Drag) + 'static>(this: *mut ffi::GdkDrag, f: glib_ffi::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn drop_performed_trampoline<F: Fn(&Drag) + 'static>(this: *mut ffi::GdkDrag, f: glib_ffi::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_actions_trampoline<F: Fn(&Drag) + 'static>(this: *mut ffi::GdkDrag, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_display_trampoline<F: Fn(&Drag) + 'static>(this: *mut ffi::GdkDrag, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_selected_action_trampoline<F: Fn(&Drag) + 'static>(this: *mut ffi::GdkDrag, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

impl fmt::Display for Drag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Drag")
    }
}