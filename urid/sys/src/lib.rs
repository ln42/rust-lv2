//! Binding of the C API for the [URID specification of LV2](http://lv2plug.in/doc/html/group__urid.html).
//!
//! Since this crate usese `bindgen` to create the C API bindings, you need to have clang installed on your machine.
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(clippy::all)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::{
    LV2_URID_Map, LV2_URID_Map_Handle, LV2_URID_Unmap, LV2_URID_Unmap_Handle, LV2_URID__map,
    LV2_URID__unmap, _LV2_URID_Map, _LV2_URID_Unmap, LV2_URID, LV2_URID_MAP_URI, LV2_URID_PREFIX,
    LV2_URID_UNMAP_URI, LV2_URID_URI,
};