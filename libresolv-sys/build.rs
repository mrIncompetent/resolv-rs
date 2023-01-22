extern crate bindgen;

use std::env;
use std::fmt::Debug;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ResolvCallbacks {}

impl bindgen::callbacks::ParseCallbacks for ResolvCallbacks {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        return match original_item_name {
            "__res_ninit" => Some(original_item_name.trim_start_matches("__").to_owned()),
            "__res_nquery" => Some(original_item_name.trim_start_matches("__").to_owned()),
            "__res_nsearch" => Some(original_item_name.trim_start_matches("__").to_owned()),
            _ => Some(original_item_name.to_owned()),
        };
    }
}

fn main() {
    println!("cargo:rustc-link-search=/usr/lib");

    println!("cargo:rustc-link-lib=resolv");

    let bindings = bindgen::Builder::default()
        .header("/usr/include/resolv.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(Box::new(ResolvCallbacks {}))
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("resolv.rs"))
        .expect("Couldn't write bindings!");
}
