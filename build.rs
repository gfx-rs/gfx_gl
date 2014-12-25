extern crate gl_generator;
extern crate khronos_api;

use std::os;
use std::io::File;

fn main() {
    let dest = Path::new(os::getenv("OUT_DIR").unwrap());

    let mut file = File::create(&dest.join("gl_bindings.rs")).unwrap();

    gl_generator::generate_bindings(gl_generator::StructGenerator,
                                                   gl_generator::registry::Ns::Gl,
                                                   khronos_api::GL_XML,
                                                   vec!["GL_EXT_texture_filter_anisotropic".to_string()],
                                                   "4.5", "core",
                                                   &mut file).unwrap();
}
