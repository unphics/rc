mod vk_bindings {
    include!(concat!(env!("OUT_DIR"), "/vulkan_bindings.rs"));
}
pub use vk_bindings::*;