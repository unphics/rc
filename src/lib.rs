#![allow(non_snake_case)] // 允许使用非蛇形命名的标识符
#![allow(dead_code)] // 允许未使用的代码
#![allow(unused_variables)] // 允许未使用的变量
#![allow(nonstandard_style)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

mod memory;
pub mod sdl2;
pub mod vk;
mod imgui;
mod math;
mod obj;

pub use memory::alloc::malloc;
pub use memory::alloc::free;
pub use memory::alloc::deref;
pub use sdl2::*;
pub use vk::*;
pub use imgui::*;
pub use math::vec2::vec2;
pub use math::vec3::vec3;
pub use math::vec2f;
pub use math::vec2i;
pub use math::vec3f;
pub use math::vec3i;
pub use math::clamp;
pub use obj::model;