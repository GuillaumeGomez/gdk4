// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Display;
use Surface;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct DrawContext(Object<ffi::GdkDrawContext, DrawContextClass>);

    match fn {
        get_type => || ffi::gdk_draw_context_get_type(),
    }
}

pub const NONE_DRAW_CONTEXT: Option<&DrawContext> = None;

pub trait DrawContextExt: 'static {
    //fn begin_frame(&self, region: /*Ignored*/&cairo::Region);

    fn end_frame(&self);

    fn get_display(&self) -> Option<Display>;

    //fn get_frame_region(&self) -> /*Ignored*/Option<cairo::Region>;

    fn get_surface(&self) -> Option<Surface>;

    fn is_in_frame(&self) -> bool;

    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DrawContext>> DrawContextExt for O {
    //fn begin_frame(&self, region: /*Ignored*/&cairo::Region) {
    //    unsafe { TODO: call ffi::gdk_draw_context_begin_frame() }
    //}

    fn end_frame(&self) {
        unsafe {
            ffi::gdk_draw_context_end_frame(self.as_ref().to_glib_none().0);
        }
    }

    fn get_display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_draw_context_get_display(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_frame_region(&self) -> /*Ignored*/Option<cairo::Region> {
    //    unsafe { TODO: call ffi::gdk_draw_context_get_frame_region() }
    //}

    fn get_surface(&self) -> Option<Surface> {
        unsafe {
            from_glib_none(ffi::gdk_draw_context_get_surface(self.as_ref().to_glib_none().0))
        }
    }

    fn is_in_frame(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_draw_context_is_in_frame(self.as_ref().to_glib_none().0))
        }
    }

    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::display\0".as_ptr() as *const _,
                Some(transmute(notify_display_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_display_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GdkDrawContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DrawContext> {
    let f: &F = &*(f as *const F);
    f(&DrawContext::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DrawContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DrawContext")
    }
}