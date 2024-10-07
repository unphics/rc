use rc::*;
use std::ffi::{CString, CStr};
use std::ptr;

fn main() {
    // 1. 创建 Vulkan 实例
    let app_name = CString::new("Hello Vulkan").unwrap();
    let engine_name = CString::new("No Engine").unwrap();

    let app_info = VkApplicationInfo {
        sType: VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
        pNext: ptr::null(),
        pApplicationName: app_name.as_ptr(),
        applicationVersion: 2,
        pEngineName: engine_name.as_ptr(),
        engineVersion: 3,
        apiVersion: VK_VERSION_1_0,
    };

    let instance_create_info = VkInstanceCreateInfo {
        sType: VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pNext: ptr::null(),
        flags: 0,
        pApplicationInfo: &app_info,
        enabledLayerCount: 0,
        ppEnabledLayerNames: ptr::null(),
        enabledExtensionCount: 0,
        ppEnabledExtensionNames: ptr::null(),
    };

    let mut instance: rc::vk::vk_bindings::VkInstance = ptr::null_mut();
    let result = unsafe {
        vkCreateInstance(&instance_create_info, ptr::null(), &mut instance)
    };

    if result != VkResult_VK_SUCCESS {
        eprintln!("Failed to create Vulkan instance: {}", result);
        return;
    }

    println!("Vulkan instance created successfully!");

    // 2. 清理资源
    unsafe {
        vkDestroyInstance(instance, ptr::null());
    }
}