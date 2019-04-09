extern crate gl_generator;

use gl_generator::{Api, Fallbacks, Profile, Registry};
use std::{env, fs::File, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let dest = Path::new(&out_dir);

    let mut file = File::create(&dest.join("gl_bindings.rs")).unwrap();

    Registry::new(
        Api::Gl,
        (4, 6),
        Profile::Core,
        Fallbacks::All,
        [
            "GL_EXT_texture_filter_anisotropic",
            "GL_ARB_draw_buffers_blend",
            "GL_ARB_program_interface_query",
            "GL_EXT_texture_compression_s3tc",
            "GL_EXT_texture_sRGB",
        ],
    )
    .write_bindings(gl_generator::StructGenerator, &mut file)
    .unwrap();
}
