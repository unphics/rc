use rc::*;
use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::mem::MaybeUninit;

const validation_layers: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];
const enable_validation_layers: bool = true;
static mut g_Instance: rc::vk::vk_bindings::VkInstance = std::ptr::null_mut();
static mut g_PhysicalDevice: rc::vk::vk_bindings::VkPhysicalDevice = std::ptr::null_mut();
static mut g_QueueFamily: u32 = u32::MAX;
static mut g_Allocator: *mut VkAllocationCallbacks = std::ptr::null_mut();
static mut g_Device: rc::vk::vk_bindings::VkDevice = std::ptr::null_mut();
static mut g_Queue: rc::vk::vk_bindings::VkQueue = std::ptr::null_mut();

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
        unsafe{extension_names.set_len(extension_count as usize);}
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

fn check_validation_support() -> bool {
    let mut layer_count: u32 = 0;
    unsafe{vkEnumerateInstanceLayerProperties(&mut layer_count, std::ptr::null_mut());}
    let mut available_layers: Vec<VkLayerProperties> = Vec::with_capacity(layer_count as usize);
    unsafe{available_layers.set_len(layer_count as usize);}
    unsafe{vkEnumerateInstanceLayerProperties(&mut layer_count, available_layers.as_mut_ptr());}
    // let validation_layers: Vec<CString> = vec![CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
    for &layer_name in validation_layers.iter() {
        let layer_name_cstr = CString::new(layer_name).unwrap();
        let mut layer_found = false;

        for layer_property in &available_layers {
            let available_layer_name = unsafe { CStr::from_ptr(layer_property.layerName.as_ptr()) };
            if layer_name_cstr.as_c_str() == available_layer_name {
                layer_found = true;
                break;
            }
        }

        if !layer_found {
            return false;
        }
    }
    return true;
}

fn setup_vulkan(mut inst_extension_names: Vec<*const ::std::os::raw::c_char>) {
    let mut err: VkResult;
    if enable_validation_layers && !check_validation_support() {
        eprintln!("validtion layers requested, but not available!");
    }
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
        // unsafe{properties.set_len(properties_count as usize );}
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

        // enabling validation layers [#ifdef APP_USE_VULKAN_DEBUG_REPORT] 这个扩展有问题先注释了
        // let layers = [CString::new("VK_LAYER_KHRONOS_validation").unwrap(),];
        // let layer_ptrs: Vec<*const c_char> = layers.iter().map(|layers| layers.as_ptr()).collect();
        // let layer_ptrs_ptr: *const *const c_char = layer_ptrs.as_ptr();
        // create_info.enabledLayerCount = 1;
        // create_info.ppEnabledLayerNames = layer_ptrs_ptr;
        // inst_extension_names.push(CString::new("VK_EXT_debug_report").unwrap().as_ptr());

        // create vulkan instance
        create_info.enabledExtensionCount = inst_extension_names.len() as u32;
        create_info.ppEnabledExtensionNames = inst_extension_names.as_mut_ptr();
        err = unsafe{vkCreateInstance(&create_info, g_Allocator, &mut g_Instance)};
        check_vk_result(err);


        let pfn = unsafe{vkGetInstanceProcAddr(g_Instance, CString::new("vkDestroyDebugReportCallbackEXT").expect("failed to new cstirng").as_ptr()).unwrap()} as *mut std::os::raw::c_void; 
        unsafe {
            let destroy_debug_report_callback: PFN_vkDestroyDebugReportCallbackEXT = if !pfn.is_null() {
                Some(std::mem::transmute::<_, unsafe extern "C" fn(rc::vk::vk_bindings::VkInstance, VkDebugReportCallbackEXT, *const VkAllocationCallbacks)>(pfn))
            } else {
                None
            };
            if let Some(destroy_fn) = destroy_debug_report_callback {
                // destroy_fn(instance, callback, allocator);  // 确保您已定义这些参数
            } else {
                eprintln!("Failed to get vkDestroyDebugReportCallbackEXT function pointer");
            }
        }
    }

    { // create validation layer

    }
    
    unsafe {g_PhysicalDevice = set_up_vulkan_select_physics_deivce();} // Select Physical Device (GPU)
    assert!(unsafe{g_PhysicalDevice} != std::ptr::null_mut());
    
    { // Select graphics queue family
        let mut count:u32 = 0;
        unsafe {vkGetPhysicalDeviceQueueFamilyProperties(g_PhysicalDevice, &mut count, std::ptr::null_mut())};
        
        let mut queues: Vec<VkQueueFamilyProperties> = Vec::with_capacity(count as usize);
        unsafe{queues.set_len(count as usize);}
        unsafe {vkGetPhysicalDeviceQueueFamilyProperties(g_PhysicalDevice, &mut count, queues.as_mut_ptr())};

        for i in 0..count-1 {
            if queues[i as usize].queueFlags & (VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT as u32) != 0{
                unsafe{g_QueueFamily = i};
                break;
            }
        }
        assert!(unsafe{g_QueueFamily} != u32::MAX);
    }

    { // create logical device
        let mut device_extensions: Vec<*const c_char> = Vec::new();
        device_extensions.push(VK_KHR_SWAPCHAIN_EXTENSION_NAME.as_ptr() as *const c_char);
        println!("222");
        // enumerate physical device extension
        let mut properties_count: u32 = 0;
        unsafe{vkEnumerateDeviceExtensionProperties(g_PhysicalDevice, std::ptr::null_mut(), &mut properties_count, std::ptr::null_mut())};
        let mut properties: Vec<VkExtensionProperties> = Vec::with_capacity(properties_count as usize);
        unsafe{properties.set_len(properties_count as usize );}
        unsafe{vkEnumerateDeviceExtensionProperties(g_PhysicalDevice, std::ptr::null_mut(), &mut properties_count, properties.as_mut_ptr())};

        let mut queue_priority: [f32; 1] = [1.0];
        let mut queue_info: [VkDeviceQueueCreateInfo; 1] = unsafe {MaybeUninit::zeroed().assume_init()};
        queue_info[0].sType = VkStructureType_VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
        queue_info[0].queueFamilyIndex = unsafe{g_QueueFamily};
        queue_info[0].queueCount = 1;
        queue_info[0].pQueuePriorities = queue_priority.as_mut_ptr(); 
        let mut create_info: VkDeviceCreateInfo = unsafe {MaybeUninit::zeroed().assume_init()};
        create_info.sType = VkStructureType_VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
        create_info.queueCreateInfoCount = queue_info.len() as u32;
        create_info.enabledExtensionCount = device_extensions.len() as u32;
        create_info.ppEnabledExtensionNames = device_extensions.as_mut_ptr();
        println!("333");
        err = unsafe{vkCreateDevice(g_PhysicalDevice, &mut create_info, g_Allocator, &mut g_Device)};

        println!("444");
        check_vk_result(err);
        unsafe{vkGetDeviceQueue(g_Device, g_QueueFamily, 0, &mut g_Queue)}
    }
}

fn set_up_vulkan_select_physics_deivce() ->VkPhysicalDevice {
    let mut gpu_count:u32 = 0;
    let mut err: VkResult;
    err = unsafe{vkEnumeratePhysicalDevices(g_Instance, &mut gpu_count, std::ptr::null_mut())};
    check_vk_result(err);
    assert!(gpu_count > 0);
    
    let mut gpus: Vec<VkPhysicalDevice> = Vec::with_capacity(gpu_count as usize);
    unsafe {gpus.set_len(gpu_count as usize);}
    err = unsafe{vkEnumeratePhysicalDevices(g_Instance, &mut gpu_count, gpus.as_mut_ptr())};
    println!("发现gpu{}个, gpu数组长度{}, 查询结果{}", gpu_count, gpus.len(), err);
    check_vk_result(err);

    for device in &gpus {
        let mut properties: VkPhysicalDeviceProperties = unsafe {MaybeUninit::zeroed().assume_init()};
        unsafe {vkGetPhysicalDeviceProperties(*device, &mut properties)};
            
        if properties.deviceType == VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU {
            return *device
        }
    }
    if gpus.len() > 0 {
        return gpus[0];
    }
    return std::ptr::null_mut();
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
