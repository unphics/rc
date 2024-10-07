/**
 * @todo 现在链接sdl2的时候需要把库拷贝到target/debug里,后面用fs直接拷过来
 */
extern crate bindgen;
use std::env;
use std::path::PathBuf;

static LIBCLANG_PATH: &str = "D:/Scoop/apps/llvm/current/lib";
static SDL2_INCLUDE_PATH: &str = "./sdl2/include";
static SDL2_LIB_PATH: &str = "./sdl2/build/Release";
static VULKAN_EXTERN_PATH: &str = "D:/Lib/VulkanSDK/1.3.290.0/Include/vulkan";
static VULKAN_INCLUDE_PATH: &str = "D:/Lib/VulkanSDK/1.3.290.0/Include/";
static VULKAN_LIB_PATH: &str = "D:/Lib/VulkanSDK/1.3.290.0/Lib";
static IMGUI_INCLUDE_PATH: &str = "./imgui/include";
static IMGUI_LIB_PATH: &str = "./imgui/build/windows/x64/release";

fn main() {
    find_libclang();
    gen_sdl2_binding();
    link_sdl2_lib();
    gen_vulkan_bindings();
    link_vulkan_lib();
    gen_imgui_bindings();
    link_imgui_lib();
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
    // println!("cargo:rerun-if-changed={}", SDL2_INCLUDE_PATH);
    // 使用 bindgen 生成 SDL2 绑定
    let bindings = bindgen::Builder::default()
        .header(format!("{}/SDL.h", SDL2_INCLUDE_PATH)) // 生成的 Rust 绑定基于 SDL2 的头文件
        .header(format!("{}/SDL_vulkan.h", SDL2_INCLUDE_PATH))
        .allowlist_function("SDL_.*")  // 只生成匹配 `SDL_` 前缀的函数绑定
        .allowlist_type("SDL_.*")
        .allowlist_var("SDL_.*")
        .raw_line("#[allow(non_snake_case)]") // 允许使用非蛇形命名的标识符
        .raw_line("#[allow(dead_code)]")// 允许未使用的代码
        .raw_line("#[allow(unused_variables)]")// 允许未使用的变量
        .generate()
        .expect("Unable to generate bindings");
    // 将生成的绑定输出到项目的 src 目录中
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("sdl2_bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn link_sdl2_lib() {
    println!("cargo:rustc-link-search=native={}", SDL2_LIB_PATH);
    println!("cargo:rustc-link-lib=static=SDL2");
}

fn gen_vulkan_bindings() {
    // 获取输出目录 (OUT_DIR)，用于保存生成的绑定文件
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // 生成 Vulkan 绑定
    let bindings = bindgen::Builder::default()
        .header(format!("{}/vulkan.h", VULKAN_EXTERN_PATH)) // 指定vulkan.h的头文件路径
        .clang_arg(format!("-I{}", VULKAN_INCLUDE_PATH)) // 指定包含路径
        .raw_line("#[allow(non_snake_case)]")
        .raw_line("#[allow(dead_code)]")
        .raw_line("#[allow(unused_variables)]")
        .generate()
        .expect("Unable to generate Vulkan bindings");
    // 将生成的绑定文件写入 OUT_DIR
    bindings
        .write_to_file(out_path.join("vulkan_bindings.rs"))
        .expect("Couldn't write bindings!");
    // 如果 Vulkan SDK 头文件发生变化，则重新运行构建脚本
    // println!("cargo:rerun-if-changed=path/to/vulkan/include/vulkan/vulkan.h");
}

fn link_vulkan_lib() {
    // println!("cargo:rustc-link-lib=vulkan");
    // // 设置链接库，链接 Vulkan 动态库
    println!("cargo:rustc-link-search=native={}", VULKAN_LIB_PATH);
    println!("cargo:rustc-link-lib=static=vulkan-1");
}

fn gen_imgui_bindings() {
    // println!("cargo:rerun-if-changed={}", IMGUI_INCLUDE_PATH);
    let bindings = bindgen::Builder::default()
        .header(format!("{}/imgui.h", IMGUI_INCLUDE_PATH))
        .clang_arg("-x")      // 告诉 clang 使用 C++ 语言
        .clang_arg("c++")     // 明确 C++ 语言
        .clang_arg("-std=c++17")  // 设置 C++ 标准为 C++11
        .raw_line("#[allow(non_snake_case)]")
        .raw_line("#[allow(dead_code)]")
        .raw_line("#[allow(unused_variables)]")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("imgui_bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn link_imgui_lib() {
    println!("cargo:rustc-link-search=native={}", IMGUI_LIB_PATH);
    println!("cargo:rustc-link-lib=static=imgui");
}