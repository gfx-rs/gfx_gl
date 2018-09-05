extern crate gl_generator;

use gl_generator::{Registry, Api, Profile, Fallbacks};
use std::env;
use std::fs::File;
use std::path::Path;
use std::collections::BTreeMap;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let dest = Path::new(&out_dir);

    let mut file = File::create(&dest.join("gl_bindings.rs")).unwrap();
    let mut alliases = BTreeMap::new();

    alliases.insert("BindVertexArray", vec!["BindVertexArrayARB"]);
    alliases.insert("DeleteVertexArrays", vec!["DeleteVertexArrays"]);
    alliases.insert("GenVertexArrays", vec!["GenVertexArraysARB"]);
    alliases.insert("IsVertexArray", vec!["IsVertexArrayARB"]);

    let alliases = alliases
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.into_iter().map(|v| v.to_string()).collect()))
        .collect();

    Registry::new(
        Api::Gl, (4, 5), 
        Profile::Core, 
        Fallbacks::All, 
        [
            "GL_EXT_texture_filter_anisotropic",
            "GL_ARB_draw_buffers_blend",
            "GL_ARB_program_interface_query",
            "GL_ARB_vertex_array_object",
        ],
        alliases,
    )
        .write_bindings(gl_generator::StructGenerator, &mut file)
        .unwrap();
}
