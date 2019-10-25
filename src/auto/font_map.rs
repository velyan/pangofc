// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use fontconfig_sys::fontconfig as fontconfig;
use glib::translate::*;
use pango_fc_sys;
use std::fmt;

glib_wrapper! {
    pub struct FontMap(Object<pango_fc_sys::PangoFcFontMap, pango_fc_sys::PangoFcFontMapClass, FontMapClass>);

    match fn {
        get_type => || pango_fc_sys::pango_fc_font_map_get_type(),
    }
}

impl FontMap {
    //pub fn add_decoder_find_func(&self, findfunc: /*Unimplemented*/Fn(/*Ignored*/fontconfig::Pattern) -> /*Ignored*/Decoder, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call pango_fc_sys:pango_fc_font_map_add_decoder_find_func() }
    //}

    pub fn cache_clear(&self) {
        unsafe {
            pango_fc_sys::pango_fc_font_map_cache_clear(self.to_glib_none().0);
        }
    }

    pub fn config_changed(&self) {
        unsafe {
            pango_fc_sys::pango_fc_font_map_config_changed(self.to_glib_none().0);
        }
    }

    //pub fn find_decoder(&self, pattern: /*Ignored*/&mut fontconfig::Pattern) -> /*Ignored*/Option<Decoder> {
    //    unsafe { TODO: call pango_fc_sys:pango_fc_font_map_find_decoder() }
    //}

    // pub fn get_config(&self) -> Option<fontconfig::FcConfig> {
    //     unsafe {
    //         Some(pango_fc_sys::pango_fc_font_map_get_config(self.to_glib_none().0))
    //     }
    // }

    pub fn set_config(&self, fcconfig: Option<&mut fontconfig::FcConfig>) {
        unsafe {
            if let Some(config) = fcconfig {
                pango_fc_sys::pango_fc_font_map_set_config(self.to_glib_none().0, config);
            }
        }
    }

    pub fn shutdown(&self) {
        unsafe {
            pango_fc_sys::pango_fc_font_map_shutdown(self.to_glib_none().0);
        }
    }
}

impl fmt::Display for FontMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FontMap")
    }
}
