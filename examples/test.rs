use rc::*;
use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;

static mut g_Instance: rc::vk::vk_bindings::VkInstance = std::ptr::null_mut();

fn main() {
    println!("===== test begin =====");
    unsafe {
        if SDL_Init(SDL_INIT_VIDEO | SDL_INIT_TIMER | SDL_INIT_GAMECONTROLLER) != 0 {
            println!("SDL init failed: {:?}", rc::sdl2::get_err());
            return;
        }
    }
    // 1.创建窗口
    let title: CString = CString::new("SDL2 begin test").unwrap();
    let window: *mut SDL_Window;
    unsafe {
        let falgs: i32 = SDL_WindowFlags_SDL_WINDOW_VULKAN | SDL_WindowFlags_SDL_WINDOW_RESIZABLE | SDL_WindowFlags_SDL_WINDOW_ALLOW_HIGHDPI;
        window = SDL_CreateWindow(title.as_ptr(), 400, 50, 800, 600, falgs as u32);
        if window.is_null() {
            println!("SDL window create failed : {:?}", rc::sdl2::get_err());
            SDL_Quit();
            return;
        }
    }
    
    // 2.扩展
    let mut extension_count = 0;
    unsafe {
        let result = SDL_Vulkan_GetInstanceExtensions(window, &mut extension_count, std::ptr::null_mut());
        if result != SDL_bool_SDL_TRUE {
            println!("failed to get extension count: {:?}", rc::sdl2::get_err());
        }
        let mut extension_names: Vec<*const ::std::os::raw::c_char> = Vec::with_capacity(extension_count as usize);
        let result = SDL_Vulkan_GetInstanceExtensions(window, &mut extension_count, extension_names.as_mut_ptr());
        if result != SDL_bool_SDL_TRUE {
            println!("failed to get vulkan instance extension_names: {:?}", rc::sdl2::get_err());
        }
        setup_vulkan(extension_names);
    }
    // 最后
    std::thread::sleep(std::time::Duration::from_secs(9));
    unsafe {
        SDL_DestroyWindow(window);
        SDL_Quit();
    }
    println!("===== test end =====");
}

fn setup_vulkan(mut inst_extension_names: Vec<*const ::std::os::raw::c_char>) {
    let mut err: VkResult;
    { // create vulkan instance
        let mut create_info:VkInstanceCreateInfo = VkInstanceCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: std::ptr::null(), // *const ::std::os::raw::c_void,
            flags: 0, // VkInstanceCreateFlags u32,
            pApplicationInfo: std::ptr::null(), // *const VkApplicationInfo,
            enabledLayerCount: 0,
            ppEnabledLayerNames: std::ptr::null(), // *const *const ::std::os::raw::c_char,
            enabledExtensionCount: 0,
            ppEnabledExtensionNames: std::ptr::null(), // *const *const ::std::os::raw::c_char,
        };
        // Enumerate available extensions
        let mut properties_count: u32 = 0;
        unsafe{vkEnumerateInstanceExtensionProperties(std::ptr::null_mut(), &mut properties_count, std::ptr::null_mut());}            
        let mut properties: Vec<VkExtensionProperties> = Vec::with_capacity(properties_count as usize);
        err = unsafe{vkEnumerateInstanceExtensionProperties(std::ptr::null_mut(), &mut properties_count, properties.as_mut_ptr())};
        check_vk_result(err);
        
        // enable required extensions
        if is_extension_available(&properties, CStr::from_bytes_with_nul(VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME).expect("Invalid CStr")) {
            inst_extension_names.push(CString::new(VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME).unwrap().as_ptr());
        }
        if is_extension_available(&properties, CStr::from_bytes_with_nul(VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME).expect("Invalid CStr")) {
            inst_extension_names.push(CString::new(VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME).unwrap().as_ptr());
            create_info.flags |= VkInstanceCreateFlagBits_VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR as u32;
        } 

        // enabling validation layers
        let layers = [CString::new("VK_LAYER_KHRONOS_validation").unwrap(),];
        let layer_ptrs: Vec<*const c_char> = layers.iter().map(|layers| layers.as_ptr()).collect();
        let layer_ptrs_ptr: *const *const c_char = layer_ptrs.as_ptr();
        create_info.enabledLayerCount = 1;
        create_info.ppEnabledLayerNames = layer_ptrs_ptr;
        inst_extension_names.push(CString::new("VK_EXT_debug_report").unwrap().as_ptr());

        // create vulkan instance
        create_info.enabledExtensionCount = inst_extension_names.len() as u32;
        create_info.ppEnabledExtensionNames = inst_extension_names.as_mut_ptr();
        err = unsafe{vkCreateInstance(& create_info, std::ptr::null(), &mut g_Instance)};
        check_vk_result(err);

        let pfn = unsafe{vkGetInstanceProcAddr(g_Instance, CString::new("vkDestroyDebugReportCallbackEXT").expect("failed to new cstirng").as_ptr()).unwrap()} as *mut std::os::raw::c_void; 
        unsafe {
            let destroy_debug_report_callback: PFN_vkDestroyDebugReportCallbackEXT = if !pfn.is_null() {
                Some(std::mem::transmute::<_, unsafe extern "C" fn(rc::vk::vk_bindings::VkInstance, VkDebugReportCallbackEXT, *const VkAllocationCallbacks)>(pfn))
            } else {
                None
            };
            // 使用这个函数指针
            if let Some(destroy_fn) = destroy_debug_report_callback {
                // 使用 destroy_fn
                // destroy_fn(instance, callback, allocator);  // 确保您已定义这些参数
            } else {
                eprintln!("Failed to get vkDestroyDebugReportCallbackEXT function pointer");
            }
        }
    }
}

fn is_extension_available(properties: &Vec<VkExtensionProperties>, extension: &CStr) -> bool {
    for p in properties {
        let cstr_ext_name = unsafe{CStr::from_ptr(p.extensionName.as_ptr())};
        if cstr_ext_name == extension {
            return true;
        }
    }
    return false;
}

fn check_vk_result(err: VkResult) {
    if err == 0 {
        return
    };
    eprintln!("Error: VkResult = {}", err);
    if err < 0 {
        std::process::exit(123);
    }
}
