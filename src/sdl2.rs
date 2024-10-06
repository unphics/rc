mod sdl2_bindings {
    include!(concat!(env!("OUT_DIR"), "/sdl2_bindings.rs"));
}
pub use sdl2_bindings::*;