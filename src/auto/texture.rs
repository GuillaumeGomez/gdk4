// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Paintable;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct Texture(Object<ffi::GdkTexture, ffi::GdkTextureClass, TextureClass>) @implements Paintable;

    match fn {
        get_type => || ffi::gdk_texture_get_type(),
    }
}

impl Texture {
    //pub fn new_for_pixbuf(pixbuf: /*Ignored*/&gdk_pixbuf::Pixbuf) -> Texture {
    //    unsafe { TODO: call ffi::gdk_texture_new_for_pixbuf() }
    //}

    //pub fn new_from_file(file: /*Ignored*/&gio::File, error: /*Ignored*/Option<Error>) -> Texture {
    //    unsafe { TODO: call ffi::gdk_texture_new_from_file() }
    //}

    pub fn new_from_resource(resource_path: &str) -> Texture {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gdk_texture_new_from_resource(resource_path.to_glib_none().0))
        }
    }
}

pub const NONE_TEXTURE: Option<&Texture> = None;

pub trait TextureExt: 'static {
    //fn download(&self, data: &[u8], stride: usize);

    fn get_height(&self) -> i32;

    fn get_width(&self) -> i32;

    fn save_to_png(&self, filename: &str) -> bool;
}

impl<O: IsA<Texture>> TextureExt for O {
    //fn download(&self, data: &[u8], stride: usize) {
    //    unsafe { TODO: call ffi::gdk_texture_download() }
    //}

    fn get_height(&self) -> i32 {
        unsafe {
            ffi::gdk_texture_get_height(self.as_ref().to_glib_none().0)
        }
    }

    fn get_width(&self) -> i32 {
        unsafe {
            ffi::gdk_texture_get_width(self.as_ref().to_glib_none().0)
        }
    }

    fn save_to_png(&self, filename: &str) -> bool {
        unsafe {
            from_glib(ffi::gdk_texture_save_to_png(self.as_ref().to_glib_none().0, filename.to_glib_none().0))
        }
    }
}

impl fmt::Display for Texture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture")
    }
}