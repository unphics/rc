use rc::*;
use std::ffi::CString;

fn get_sdl2_err_str() -> CString {
    unsafe {
        let error = SDL_GetError();
        let c_str = CString::from_raw(error as *mut i8);
        return c_str;
    }
}

fn main() {
    // 初始化 SDL2
    unsafe {
        if SDL_Init(SDL_INIT_VIDEO) != 0 {
            println!("SDL init failed: {:?}", get_sdl2_err_str());
            return;
        }
    }
    println!("SDL init succeed");
    // 创建窗口
    let title = CString::new("SDL2 begin test").unwrap();
    let window: *mut SDL_Window;
    unsafe {
        window = SDL_CreateWindow(
            title.as_ptr(),
            400, // SDL_WINDOWPOS_CENTERED as i32,
            50, // SDL_WINDOWPOS_CENTERED as i32,
            800,
            600,
            SDL_WindowFlags_SDL_WINDOW_SHOWN as u32
        );
        if window.is_null() {
            println!("SDL window create failed : {:?}", get_sdl2_err_str());
            SDL_Quit();
            return;
        }
    }
    println!("SDL window create succeed");
    std::thread::sleep(std::time::Duration::from_secs(3));
    unsafe {
        SDL_DestroyWindow(window);
        SDL_Quit();
    }
    println!("SDL window destroyed and SDL quit succeed");
}