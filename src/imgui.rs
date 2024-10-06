mod imgui_bindings {
    include!(concat!(env!("OUT_DIR"), "/imgui_bindings.rs"));
}
pub use imgui_bindings::*;