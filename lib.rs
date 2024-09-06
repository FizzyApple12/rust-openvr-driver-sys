#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

include!("bindings.rs");

#[cfg(target_os = "macos")]
#[link(name = "Foundation", kind = "framework")]
extern "C" {}
