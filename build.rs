#![feature(path)]

extern crate gl_generator;
extern crate khronos_api;

use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let dest = Path::new(&out_dir);

    let mut file = File::create(&dest.join("gl_bindings.rs")).unwrap();

    gl_generator::generate_bindings(gl_generator::StructGenerator,
                                                   gl_generator::registry::Ns::Gl,
                                                   gl_generator::Fallbacks::All,
                                                   khronos_api::GL_XML,
                                                   vec!["GL_EXT_texture_filter_anisotropic".to_string()],
                                                   "4.5", "core",
                                                   &mut file).unwrap();
}
