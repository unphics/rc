/**
 * @todo 现在链接sdl2的时候需要把库拷贝到target/debug里,后面用fs直接拷过来
 */
extern crate bindgen;
use std::env;
use std::path::PathBuf;

static LIBCLANG_PATH: &str = "D:/Scoop/apps/llvm/current/lib";
static SDL2_INCLUDE_PATH: &str = "D:/Scoop/apps/sdl2/current/include/SDL2";
static SDL2_LIB_PATH: &str = "D:/Scoop/apps/sdl2/current/lib";
static VULKAN_EXTERN_PATH: &str = "D:/Lib/VulkanSDK/1.3.290.0/Include/vulkan";
static VULKAN_INCLUDE_PATH: &str = "D:/Lib/VulkanSDK/1.3.290.0/Include/";
static VULKAN_LIB_PATH: &str = "d:/Lib/VulkanSDK/1.3.290.0/Lib";

fn main() {
    find_libclang();
    gen_sdl2_binding();
    link_sdl2();
    gen_vulkan_bindings();
    link_vulkan();
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
    println!("cargo:rustc-link-lib=dylib=SDL2");
}

fn gen_vulkan_bindings() {
    // 获取输出目录 (OUT_DIR)，用于保存生成的绑定文件
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // 生成 Vulkan 绑定
    let bindings = bindgen::Builder::default()
        .header(format!("{}/vulkan.h", VULKAN_EXTERN_PATH)) // 指定vulkan.h的头文件路径
        .clang_arg(format!("-I{}", VULKAN_INCLUDE_PATH)) // 指定包含路径
        .generate()
        .expect("Unable to generate Vulkan bindings");
    // 将生成的绑定文件写入 OUT_DIR
    bindings
        .write_to_file(out_path.join("vulkan_bindings.rs"))
        .expect("Couldn't write bindings!");
    // 如果 Vulkan SDK 头文件发生变化，则重新运行构建脚本
    println!("cargo:rerun-if-changed=path/to/vulkan/include/vulkan/vulkan.h");
}

fn link_vulkan() {
    // println!("cargo:rustc-link-lib=vulkan");
    // // 设置链接库，链接 Vulkan 动态库
    println!("cargo:rustc-link-search=native={}", VULKAN_LIB_PATH);
    println!("cargo:rustc-link-lib=static=vulkan-1");
}