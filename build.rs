/**
 * @todo 现在链接sdl2的时候需要把库拷贝到target/debug里,后面用fs直接拷过来
 */
extern crate bindgen;
use std::env;
use std::path::PathBuf;
use std::fs::File;
use serde_json::from_reader;
use serde_json::Value;

static mut config: Value = Value::Null;

fn main() {
    init_config();
    find_libclang();
    gen_sdl2_binding();
    link_sdl2_lib();
    gen_vulkan_bindings();
    link_vulkan_lib();
    gen_imgui_bindings();
    link_imgui_lib();
}

fn init_config() {
    let cfg_file = File::open("config.json").unwrap();
    unsafe {config = from_reader(cfg_file).unwrap();}
    if unsafe{config == Value::Null} || unsafe{config.is_null()} {
        eprintln!("failed to init config file");
    }
}

fn read_config(key: &str) -> String {
    String::from(unsafe{config[key].as_str().expect(format!("index a nil value by {}", key).as_str())})
}

/**
 * @brief bindgen解析cpp文件需要clang
 */
fn find_libclang() {
    // 设置 LIBCLANG_PATH 环境变量
    env::set_var("LIBCLANG_PATH", read_config("libclang_path"));
}

fn gen_sdl2_binding() {
    // println!("cargo:rerun-if-changed={}", read_config("sdl2_include_path")); // 告诉 Cargo 重新构建，如果头文件发生变化
    // 使用 bindgen 生成 SDL2 绑定
    let bindings = bindgen::Builder::default()
        .header(format!("{}/SDL.h", read_config("sdl2_include_path"))) // 生成的 Rust 绑定基于 SDL2 的头文件
        .header(format!("{}/SDL_vulkan.h", read_config("sdl2_include_path")))
        .allowlist_function("SDL_.*")  // 只生成匹配 `SDL_` 前缀的函数绑定
        .allowlist_type("SDL_.*")
        .allowlist_var("SDL_.*")
        .generate()
        .expect("Unable to generate bindings");
    // 将生成的绑定输出到项目的 src 目录中
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("sdl2_bindings.rs")).expect("Couldn't write bindings!");
}

fn link_sdl2_lib() {
    println!("cargo:rustc-link-search=native={}", read_config("sdl2_lib_path"));
    println!("cargo:rustc-link-lib=static=SDL2");
}

fn gen_vulkan_bindings() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()); // 获取输出目录 (OUT_DIR)，用于保存生成的绑定文件
    // 生成 Vulkan 绑定
    let bindings = bindgen::Builder::default()
        .header(format!("{}/vulkan.h", read_config("vulkan_extern_path"))) // 指定vulkan.h的头文件路径
        .clang_arg(format!("-I{}", read_config("vulkan_include_path"))) // 指定包含路径
        .generate()
        .expect("Unable to generate Vulkan bindings");
    bindings.write_to_file(out_path.join("vulkan_bindings.rs")).expect("Couldn't write bindings!"); // 将生成的绑定文件写入 OUT_DIR
    // println!("cargo:rerun-if-changed=path/to/vulkan/include/vulkan/vulkan.h"); // 如果 Vulkan SDK 头文件发生变化，则重新运行构建脚本
}

fn link_vulkan_lib() {
    // println!("cargo:rustc-link-lib=vulkan");
    // 设置链接库，链接 Vulkan 动态库
    println!("cargo:rustc-link-search=native={}", read_config("vulkan_lib_path"));
    println!("cargo:rustc-link-lib=static=vulkan-1");
}

fn gen_imgui_bindings() {
    // println!("cargo:rerun-if-changed={}", read_config("imgui_include_path"));
    let bindings = bindgen::Builder::default()
        .header(format!("{}/imgui.h", read_config("imgui_include_path")))
        .clang_arg("-x")      // 告诉 clang 使用 C++ 语言
        .clang_arg("c++")     // 明确 C++ 语言
        .clang_arg("-std=c++17")  // 设置 C++ 标准为 C++11
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("imgui_bindings.rs")).expect("Couldn't write bindings!");
}

fn link_imgui_lib() {
    println!("cargo:rustc-link-search=native={}", read_config("imgui_lib_path"));
    println!("cargo:rustc-link-lib=static=imgui");
}