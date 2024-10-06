/**
 * @todo 现在链接sdl2的时候需要把库拷贝到target/debug里,后面用fs直接拷过来
 */
extern crate bindgen;
use std::env;
use std::path::PathBuf;

static LIBCLANG_PATH: &str = "D:/Scoop/apps/llvm/current/lib";
static SDL2_INCLUDE_PATH: &str = "D:/Scoop/apps/sdl2/current/include/SDL2";
static SDL2_LIB_PATH: &str = "D:/Scoop/apps/sdl2/current/lib";

fn main() {
    find_libclang();
    gen_sdl2_binding();
    link_sdl2();
}

/**
 * @brief bindgen解析cpp文件需要clang, 故需要scoop install clang
 */
fn find_libclang() {
    // 设置 LIBCLANG_PATH 环境变量
    env::set_var("LIBCLANG_PATH", LIBCLANG_PATH);
}

fn gen_sdl2_binding() {
    // 告诉 Cargo 重新构建，如果头文件发生变化
    println!("cargo:rerun-if-changed={}", SDL2_INCLUDE_PATH);
    // 使用 bindgen 生成 SDL2 绑定
    let bindings = bindgen::Builder::default()
        .header(format!("{}/SDL.h", SDL2_INCLUDE_PATH)) // 生成的 Rust 绑定基于 SDL2 的头文件
        .allowlist_function("SDL_.*")  // 只生成匹配 `SDL_` 前缀的函数绑定
        .allowlist_type("SDL_.*")
        .allowlist_var("SDL_.*")
        .generate()
        .expect("Unable to generate bindings");
    // 将生成的绑定输出到项目的 src 目录中
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("sdl2_bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn link_sdl2() {
    println!("cargo:rustc-link-search=native={}", SDL2_LIB_PATH);
    println!("cargo:rustc-link-lib=static=SDL2");
}