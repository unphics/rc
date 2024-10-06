mod memory;
mod sdl2;
pub use memory::alloc::malloc;
pub use memory::alloc::free;
pub use memory::alloc::deref;
pub use sdl2::*;