mod sdl2_bindings {include!(concat!(env!("OUT_DIR"), "/sdl2_bindings.rs"));}
pub use sdl2_bindings::*;
use std::ffi::CString;

pub const SDL_INIT_VIDEO: u32 = 32;

pub fn init(flags: u32) -> ::std::os::raw::c_int {
    unsafe {return SDL_Init(flags);}
}

pub fn get_err() -> String {
    let err: String;
    unsafe {
        let error = SDL_GetError();
        let c_str = CString::from_raw(error as *mut i8);
        err = String::from(c_str.to_str().unwrap());
    }
    return err;
}