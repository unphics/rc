use rc::*;
use std::ffi::CString;
use image::{self, ImageBuffer, Rgb};

const WindowDuration: u64 = 20;

fn main() {
    unsafe { // 初始化 SDL2
        if SDL_Init(SDL_INIT_VIDEO) != 0 {
            println!("SDL init failed: {:?}", get_sdl2_err_str());
            return;
        }
        println!("SDL init succeed");
    }
    let window: *mut SDL_Window;
    unsafe {
        // 创建窗口
        let title = CString::new("SDL2 begin test").unwrap();
        window = SDL_CreateWindow(
            title.as_ptr(),
            400, // SDL_WINDOWPOS_CENTERED as i32,
            50, // SDL_WINDOWPOS_CENTERED as i32,
            1600,
            1200,
            SDL_WindowFlags_SDL_WINDOW_SHOWN as u32
        );
        if window.is_null() {
            println!("SDL window create failed : {:?}", get_sdl2_err_str());
            SDL_DestroyWindow(window);
            SDL_Quit();
            return;
        }
        println!("SDL window create succeed");
    }
    {
        let width = 1600;
        let height = 1200;
        let mut img_buf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
        draw(&mut img_buf);
        img_buf.save("resource/new.bmp").unwrap();
    }
    let rwops: *mut SDL_RWops;
    unsafe { // 打开bmp文件
        let img_path = CString::new("resource/new.bmp").unwrap();
        rwops = SDL_RWFromFile(img_path.as_ptr(), CString::new("rb").unwrap().as_ptr());
        if rwops.is_null() {
            println!("SDL rwops create failed : {:?}", get_sdl2_err_str());
            SDL_DestroyWindow(window);
            SDL_Quit();
            return;
        }
    }
    let surface: *mut SDL_Surface;
    unsafe {
        surface = SDL_LoadBMP_RW(rwops, 1); // 第二个参数 `1` 表示关闭 RWops
        if surface.is_null() {
            println!("SDL surface create failed : {:?}", get_sdl2_err_str());
            SDL_DestroyWindow(window);
            SDL_Quit();
            return;
        }
    }
    let renderer: *mut SDL_Renderer;
    unsafe { // 创建渲染器
        renderer = SDL_CreateRenderer(window, -1, SDL_RendererFlags_SDL_RENDERER_ACCELERATED as u32 | SDL_RendererFlags_SDL_RENDERER_PRESENTVSYNC as u32);
        if renderer.is_null() {
            println!("SDL renderer create failed : {:?}", get_sdl2_err_str());
            SDL_DestroyWindow(window);
            SDL_Quit();
            return;
        }
    }
    let texture: *mut SDL_Texture;
    unsafe{ // 创建纹理
        texture = SDL_CreateTextureFromSurface(renderer, surface); // 创建纹理
        SDL_FreeSurface(surface); // 释放surface
        if texture.is_null() {
            println!("SDL texture create failed : {:?}", get_sdl2_err_str());
            SDL_DestroyRenderer(renderer);
            SDL_DestroyWindow(window);
            SDL_Quit();
            return;
        }
    }
    unsafe{
        SDL_RenderClear(renderer); // 清空渲染器
        SDL_RenderCopy(renderer, texture, std::ptr::null(), std::ptr::null()); // 将纹理复制到渲染器中
        SDL_RenderPresent(renderer); // 呈现渲染器内容
    }
    std::thread::sleep(std::time::Duration::from_secs(WindowDuration));
    unsafe {
        SDL_DestroyWindow(window);
        SDL_Quit();
        println!("SDL window destroyed and SDL quit succeed");
    }
}

fn get_sdl2_err_str() -> CString {
    unsafe {
        let error = SDL_GetError();
        let c_str = CString::from_raw(error as *mut i8);
        return c_str;
    }
}
type ImgBuf = ImageBuffer<Rgb<u8>, Vec<u8>>;
type Color = Rgb<u8>;
fn draw(img: &mut ImgBuf) { // 填充像素
    for (_, _, pixel) in img.enumerate_pixels_mut() {
        *pixel = Rgb([0, 0, 0]);
    }
    // line(13,20, 80, 40, img, Rgb([255, 0, 0]));
    // line(20,13, 40, 80, img, Rgb([255, 0, 0]));
    // line(80,40, 13, 20, img, Rgb([255, 0, 0]));
    // line(0,100, 100, 100, img, Rgb([255, 0, 0]));
    // line(100,0, 100, 100, img, Rgb([0, 255, 0]));

    let model = rc::model::new("resource/african_head.obj").unwrap();
    for i in 0..model.nfaces() {
        let face = model.face(i);
        for j in 0..2 {
            let v0 = model.vert(face[j] as usize);
            let v1 = model.vert(face[(j + 1) % 3] as usize);
            let x0 = ((v0.x + 1f32) * (800 / 2) as f32) as i64;
            let y0 = ((v0.y + 1f32) * (800 / 2) as f32) as i64;
            let x1 = ((v1.x + 1f32) * (800 / 2) as f32) as i64;
            let y1 = ((v1.y + 1f32) * (800 / 2) as f32) as i64;
            line(x0, y0, x1, y1, img, Rgb([255, 255, 255]));
        }
    }
}
fn set_color(x: i64, y: i64, img: &mut ImgBuf,color: Color) {
    *img.get_pixel_mut(x as u32, y as u32) = color;
}
// 后续优化
fn line(mut x0: i64, mut y0: i64, mut x1: i64, mut y1:i64, img: &mut ImgBuf, color: Color) {
    let mut steep: bool = false;
    if (x0 as f32 - x1 as f32).abs() < (y0 as f32 - y1 as f32).abs() {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
        steep = true;
    }
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }
    if x1 != x0 {
        for x in x0..x1 {
            let y: i64 = y0 + (y1 - y0) * ((x - x0) / (x1 - x0));
            if steep {
                set_color(y, x, img, color);
            } else {
                set_color(x, y, img, color);
            }
        }
    }
}
/* ref[1]
y = y0 + (y1 - y0) * t
  = y0 + y1 * t - y0 * t
  = y0 - y0 * t + y1 * t
  = y0 * (1 - t) + y1 * t
*/