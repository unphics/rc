#![allow(non_snake_case)] // 允许使用非蛇形命名的标识符
#![allow(non_camel_case_types)] // 允许使用非驼峰命名的类型
#![allow(non_upper_case_globals)] // 允许使用非大写字母的全局变量
#![allow(nonstandard_style)] 

use rc::*;
use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::mem::MaybeUninit;
use rc::vk::vk_bindings as vk;
use std::option::Option;
use std::collections::HashSet;

const Width: i32 = 800;
const Height: i32 = 600;
const validation_layers: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];
const enable_validation_layers: bool = true;
const device_extensions: [*const i8; 1] = [vk::VK_KHR_SWAPCHAIN_EXTENSION_NAME.as_ptr() as *const c_char];

struct queue_family_indices {
    pub graphics_family: Option<u32>,
    pub preset_family: Option<u32>,
}
impl queue_family_indices {
    pub fn new() -> Self {
        return Self {
            graphics_family: None,
            preset_family: None,
        }
    }
    pub fn is_complete(&self) -> bool {
        return self.graphics_family.is_some() && self.preset_family.is_some();
    }
}

struct swap_chain_support_details {
    capabilities: vk::VkSurfaceCapabilitiesKHR,
    formats: Vec<vk::VkSurfaceFormatKHR>,
    present_modes: Vec<vk::VkPresentModeKHR>,
}

struct app {
    window: *mut SDL_Window,
    instance: vk::VkInstance,
    debug_messenger: vk::VkDebugUtilsMessengerEXT,
    physical_device: vk::VkPhysicalDevice,
    device: vk::VkDevice,
    graphics_queue: vk::VkQueue,
    surface: vk::VkSurfaceKHR,
    preset_queue: vk::VkQueue,
    swap_chain: vk::VkSwapchainKHR,
    swap_chain_images: Vec<vk::VkImage>,
    swap_chain_image_format: vk::VkFormat,
    swap_chain_extent: vk::VkExtent2D,
    swap_chain_image_views: Vec<vk::VkImageView>,
}
impl app {
    pub fn new() -> Self {
        Self {
            window: std::ptr::null_mut(),
            instance: std::ptr::null_mut(),
            debug_messenger: std::ptr::null_mut(),
            physical_device: std::ptr::null_mut(),
            device: std::ptr::null_mut(),
            graphics_queue: std::ptr::null_mut(),
            surface: std::ptr::null_mut(),
            preset_queue: std::ptr::null_mut(),
            swap_chain: std::ptr::null_mut(),
            swap_chain_images: Vec::new(),
            swap_chain_image_format: 0,
            swap_chain_extent: unsafe {MaybeUninit::zeroed().assume_init()},
            swap_chain_image_views: Vec::new(),
        }
    }
    pub fn run(&mut self) {
        self.init_window();
        self.init_vulkan();
        self.main_loop();
        self.clean_up();
    }
    fn init_window(&mut self) {
        unsafe {
            if SDL_Init(SDL_INIT_VIDEO | SDL_INIT_TIMER | SDL_INIT_GAMECONTROLLER) != 0 {
                panic!("SDL_Init failed: {:?}", rc::sdl2::get_err());
            }
            let title: CString = CString::new("SDL Vulkan App").unwrap();
            let flags = SDL_WindowFlags_SDL_WINDOW_VULKAN | SDL_WindowFlags_SDL_WINDOW_RESIZABLE | SDL_WindowFlags_SDL_WINDOW_ALLOW_HIGHDPI;
            self.window = SDL_CreateWindow(title.as_ptr(), 400, 50, Width, Height, flags as u32);
            if self.window.is_null() {
                SDL_Quit();
                panic!("SDL_CreateWindow failed: {:?}", rc::sdl2::get_err());
            }
        }
    }
    fn init_vulkan(&mut self) {
        self.create_instance();
        self.setup_debug_messenger();   
        self.create_suface();
        self.pick_physical_device();
        self.create_logical_device();
        self.craete_swap_chain();
        self.create_image_views();
    }
    fn main_loop(&self) {
        std::thread::sleep(std::time::Duration::from_secs(4));
    }
    fn clean_up(&mut self) {
        println!("\n----- clean_up -----");
        unsafe {
            for image_view in &self.swap_chain_image_views {
                vk::vkDestroyImageView(self.device, *image_view, std::ptr::null_mut());
            }
            vk::vkDestroySwapchainKHR(self.device, self.swap_chain, std::ptr::null_mut());
            vk::vkDestroyDevice(self.device, std::ptr::null_mut());
            if enable_validation_layers {
                self.destroy_debug_utils_messenger_ext();
            }
            vk::vkDestroySurfaceKHR(self.instance, self.surface, std::ptr::null_mut());
            rc::vk::vk_bindings::vkDestroyInstance(self.instance, std::ptr::null());
            SDL_DestroyWindow(self.window);
            SDL_Quit();
        }
    }
    fn create_instance(&mut self) {
        println!("\n----- create_instance -----");
        if enable_validation_layers && !self.check_validation_layer_support() {
            panic!("validation layers requested, but not available!");
        }
        let mut app_info: vk::VkApplicationInfo = unsafe {MaybeUninit::zeroed().assume_init()};
        app_info.sType = vk::VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO;
        app_info.pApplicationName = "SDL Vulkan App".as_ptr() as *const c_char;
        app_info.applicationVersion = 4194304;
        app_info.pEngineName = "No Engine".as_ptr() as *const c_char;
        app_info.engineVersion = 4194304;
        app_info.apiVersion = 4194304;

        let mut create_info: vk::VkInstanceCreateInfo = unsafe {MaybeUninit::zeroed().assume_init()};
        create_info.sType = vk::VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
        create_info.pApplicationInfo = &app_info;

        let extensions = self.get_required_extensions();
        create_info.enabledExtensionCount = extensions.len() as u32;
        create_info.ppEnabledExtensionNames = extensions.as_ptr();
        let mut debug_create_info: vk::VkDebugUtilsMessengerCreateInfoEXT = unsafe {MaybeUninit::zeroed().assume_init()};
        if enable_validation_layers {
            create_info.enabledLayerCount = validation_layers.len() as u32;
            create_info.ppEnabledLayerNames = validation_layers.as_ptr() as *const *const c_char;
            self.populate_debug_messenger_create_info(&mut debug_create_info);
            create_info.pNext = unsafe {std::mem::transmute(&debug_create_info)};
        } else {
            create_info.enabledLayerCount = 0;
            create_info.pNext = std::ptr::null();
        }
        if unsafe {vk::vkCreateInstance(&create_info, std::ptr::null(), &mut self.instance)} != vk::VkResult_VK_SUCCESS {
            panic!("vkCreateInstance failed");
        }
        println!("create_info.enabledExtensionCount {}", create_info.enabledExtensionCount);
    }
    fn get_required_extensions(&self) -> Vec<*const c_char> {
        // let mut extension_count = 0;
        // if unsafe{SDL_Vulkan_GetInstanceExtensions(self.window, &mut extension_count, std::ptr::null_mut())} != SDL_bool_SDL_TRUE {
        //     println!("failed to get extension count: {:?}", rc::sdl2::get_err());
        // }
        // let mut extensions: Vec<*const i8> = Vec::with_capacity(extension_count as usize);
        // unsafe{extensions.set_len(extension_count as usize);}
        // if unsafe{SDL_Vulkan_GetInstanceExtensions(self.window, &mut extension_count, extensions.as_mut_ptr())} != SDL_bool_SDL_TRUE {
        //     println!("failed to get vulkan instance extension_names: {:?}", rc::sdl2::get_err());
        // }
        // for &ext in &extensions {
        //     let c_str = unsafe { CStr::from_ptr(ext as *const c_char) };
        //     println!("看看你都用了啥{}", c_str.to_str().unwrap());
        // }

        let mut extensions: Vec<*const c_char> = Vec::new();
        {
            extensions.push(VK_KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME.as_ptr() as *const c_char);
            extensions.push(VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME.as_ptr() as *const c_char);
            extensions.push(VK_KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME.as_ptr() as *const c_char);
            extensions.push(VK_KHR_SURFACE_EXTENSION_NAME.as_ptr() as *const c_char);
            extensions.push(VK_EXT_DEBUG_REPORT_EXTENSION_NAME.as_ptr() as *const c_char);
            extensions.push(VK_EXT_DEBUG_UTILS_EXTENSION_NAME.as_ptr() as *const c_char); // **
            // extensions.push(VK_KHR_DISPLAY_EXTENSION_NAME.as_ptr() as *const c_char);
            extensions.push(VK_KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME.as_ptr() as *const c_char);
            extensions.push(VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME.as_ptr() as *const c_char);
            extensions.push(VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME.as_ptr() as *const c_char);
            // extensions.push(VK_KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME.as_ptr() as *const c_char);
            // extensions.push(VK_KHR_SURFACE_PROTECTED_CAPABILITIES_EXTENSION_NAME.as_ptr() as *const c_char);
            // extensions.push(VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME.as_ptr() as *const c_char);
            extensions.push(VK_EXT_SURFACE_MAINTENANCE_1_EXTENSION_NAME.as_ptr() as *const c_char);
            extensions.push(VK_EXT_SWAPCHAIN_COLOR_SPACE_EXTENSION_NAME.as_ptr() as *const c_char);
            extensions.push(VK_NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME.as_ptr() as *const c_char);
            extensions.push(VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME.as_ptr() as *const c_char);
            extensions.push(VK_LUNARG_DIRECT_DRIVER_LOADING_EXTENSION_NAME.as_ptr() as *const c_char);
            extensions.push(b"VK_KHR_win32_surface\0".as_ptr() as *const c_char);
        }

        {
            // let mut properties_count: u32 = 0;
            // unsafe{vkEnumerateInstanceExtensionProperties(std::ptr::null_mut(), &mut properties_count, std::ptr::null_mut());}            
            // println!("支持的扩展{}", properties_count);
            // let mut properties: Vec<VkExtensionProperties> = Vec::with_capacity(properties_count as usize);
            // unsafe{properties.set_len(properties_count as usize);}
            // let err = unsafe{vkEnumerateInstanceExtensionProperties(std::ptr::null_mut(), &mut properties_count, properties.as_mut_ptr())};
            // check_vk_result(err);
            // for prop in properties {
            //     let byte_slice: &[u8] = &prop.extensionName.iter().map(|&b| b as u8).collect::<Vec<u8>>();
            //     println!("扩展名{}", unsafe{CStr::from_bytes_with_nul_unchecked(byte_slice).to_str().unwrap()});
            //     extensions.push(unsafe{CStr::from_bytes_with_nul_unchecked(byte_slice).to_str().unwrap().as_bytes().as_ptr() as *const i8});
            // }
        }
        return extensions;
    }
    fn populate_debug_messenger_create_info(&mut self, create_info: &mut vk::VkDebugUtilsMessengerCreateInfoEXT) {
        create_info.sType = vk::VkStructureType_VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT;
        let flags = vk::VkDebugUtilsMessageSeverityFlagBitsEXT_VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT | vk::VkDebugUtilsMessageSeverityFlagBitsEXT_VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT | VkDebugUtilsMessageSeverityFlagBitsEXT_VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT;
        create_info.messageSeverity = flags as u32;
        let flags = vk::VkDebugUtilsMessageTypeFlagBitsEXT_VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT | vk::VkDebugUtilsMessageTypeFlagBitsEXT_VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT | vk::VkDebugUtilsMessageTypeFlagBitsEXT_VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT;
        create_info.messageType = flags as u32;
        create_info.pfnUserCallback = Some(debug_callback);
    }
    fn setup_debug_messenger(&mut self) {
        println!("\n----- setup_debug_messenger -----");
        if !enable_validation_layers {
            return;
        }
        let mut create_info: vk::VkDebugUtilsMessengerCreateInfoEXT = unsafe {MaybeUninit::zeroed().assume_init()};
        self.populate_debug_messenger_create_info(&mut create_info);
        self.create_debug_utils_messenger_ext(&create_info);
        // if unsafe {vk::vkCreateDebugUtilsMessengerEXT(self.instance, &create_info, std::ptr::null(), &mut self.debug_messenger)} != vk::VkResult_VK_SUCCESS {
        //     panic!("vkCreateDebugUtilsMessengerEXT failed");
        // }
    }
    fn create_debug_utils_messenger_ext(&mut self, create_info: &vk::VkDebugUtilsMessengerCreateInfoEXT) {
        //todo notice 这里很重要
        let fn_name = CStr::from_bytes_with_nul(b"vkCreateDebugUtilsMessengerEXT\0").unwrap();
        let func: vk::PFN_vkCreateDebugUtilsMessengerEXT = unsafe{std::mem::transmute(vk::vkGetInstanceProcAddr(self.instance, fn_name.as_ptr() as *const c_char))};
        if let Some(foo) = func {
            unsafe{foo(self.instance, create_info, std::ptr::null(), &mut self.debug_messenger)};
        } else {
            panic!("vkCreateDebugUtilsMessengerEXT not found");
        }
    }
    fn destroy_debug_utils_messenger_ext(&mut self) {
        let fn_name = CStr::from_bytes_with_nul(b"vkDestroyDebugUtilsMessengerEXT\0").unwrap();
        let func: vk::PFN_vkDestroyDebugUtilsMessengerEXT = unsafe{std::mem::transmute(vk::vkGetInstanceProcAddr(self.instance, fn_name.as_ptr() as *const c_char))};
        if let Some(foo) = func {
            unsafe{foo(self.instance, self.debug_messenger, std::ptr::null());}
        }
    }
    fn check_validation_layer_support(&self) -> bool {
        let mut layer_count: u32 = 0;
        unsafe{vk::vkEnumerateInstanceLayerProperties(&mut layer_count, std::ptr::null_mut());}
        let mut available_layers: Vec<vk::VkLayerProperties> = Vec::with_capacity(layer_count as usize);
        unsafe{available_layers.set_len(layer_count as usize);}
        unsafe{vk::vkEnumerateInstanceLayerProperties(&mut layer_count, available_layers.as_mut_ptr());}
        for &layer_name in validation_layers.iter() {
            let layer_name_cstr = CString::new(layer_name).unwrap();
            let mut layer_found = false;
            for layer_properties in available_layers.iter() {
                let available_layer_name = unsafe { CStr::from_ptr(layer_properties.layerName.as_ptr()) };
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
    fn pick_physical_device(&mut self) {
        println!("\n----- pick_physical_device -----");
        let mut device_count: u32 = 0;
        unsafe{vkEnumeratePhysicalDevices(self.instance, &mut device_count, std::ptr::null_mut())};
        if device_count == 0 {
            panic!("failed to find gpus with vulkan support!");
        }
        let mut devices: Vec<VkPhysicalDevice> = Vec::with_capacity(device_count as usize);
        unsafe {devices.set_len(device_count as usize);}
        unsafe{vkEnumeratePhysicalDevices(self.instance, &mut device_count, devices.as_mut_ptr())};
        for device in &devices {
            if self.is_device_suitable(device) {
                self.physical_device = *device;
                break;
            }
        }
        if self.physical_device.is_null() {
            panic!("failed to find a suitable gpu!");
        }
    }
    fn is_device_suitable(&self, device: &vk::VkPhysicalDevice) -> bool {
        let indices = self.find_queue_families(device);
        let extensions_supported = self.check_device_extension_support(device);
        let mut swap_chain_adequate = false;
        if extensions_supported {
            let swap_chain_support = self.query_swap_chain_support(device);
            swap_chain_adequate = !swap_chain_support.formats.is_empty() && !swap_chain_support.present_modes.is_empty();
        }
        return indices.is_complete() && extensions_supported && swap_chain_adequate;
    }
    fn query_swap_chain_support(&self, device: &vk::VkPhysicalDevice) -> swap_chain_support_details {
        let mut details: swap_chain_support_details = unsafe {MaybeUninit::zeroed().assume_init()};
        unsafe{vk::vkGetPhysicalDeviceSurfaceCapabilitiesKHR(*device, self.surface, &mut details.capabilities)};
        let mut format_count: u32 = 0;
        unsafe{vk::vkGetPhysicalDeviceSurfaceFormatsKHR(*device, self.surface, &mut format_count, std::ptr::null_mut())};
        if format_count != 0 {
            details.formats = Vec::with_capacity(format_count as usize);
            unsafe{details.formats.set_len(format_count as usize)};
            unsafe{vk::vkGetPhysicalDeviceSurfaceFormatsKHR(*device, self.surface, &mut format_count, details.formats.as_mut_ptr())};
        }
        let mut present_mode_count: u32 = 0;
        unsafe{vk::vkGetPhysicalDeviceSurfacePresentModesKHR(*device, self.surface, &mut present_mode_count, std::ptr::null_mut())};
        if present_mode_count != 0 {
            details.present_modes = Vec::with_capacity(present_mode_count as usize);
            unsafe{details.present_modes.set_len(present_mode_count as usize);}
            unsafe{vk::vkGetPhysicalDeviceSurfacePresentModesKHR(*device, self.surface, &mut present_mode_count, details.present_modes.as_mut_ptr())};
        }
        return details;
    }
    fn check_device_extension_support(&self, device: &vk::VkPhysicalDevice) -> bool {
        let mut extension_count = 0;
        unsafe{vk::vkEnumerateDeviceExtensionProperties(*device, std::ptr::null_mut(), &mut extension_count, std::ptr::null_mut())};
        let mut available_extensions: Vec<VkExtensionProperties> = Vec::with_capacity(extension_count as usize);
        unsafe{available_extensions.set_len(extension_count as usize);}
        unsafe{vk::vkEnumerateDeviceExtensionProperties(*device, std::ptr::null_mut(), &mut extension_count, available_extensions.as_mut_ptr())};
        let mut available_extensions_set = HashSet::new();
        for extension in available_extensions {
            let ext_name = unsafe {
                CStr::from_ptr(extension.extensionName.as_ptr())
                    .to_string_lossy()
                    .into_owned()
            };
            available_extensions_set.insert(ext_name);
        }
        for ext in device_extensions {
            let ext_name = unsafe { CStr::from_ptr(ext).to_string_lossy().into_owned() };
            println!("比较一下{}",ext_name);
            if !available_extensions_set.contains(&ext_name) {
                return false;
            }
        }
        return true;
    }
    fn find_queue_families(&self, device: &vk::VkPhysicalDevice) -> queue_family_indices {
        let mut indices = queue_family_indices::new();
        let mut queue_family_count: u32 = 0;
        unsafe{vk::vkGetPhysicalDeviceQueueFamilyProperties(*device, &mut queue_family_count, std::ptr::null_mut())}
        let mut queue_families: Vec<VkQueueFamilyProperties> = Vec::with_capacity(queue_family_count as usize);
        unsafe{queue_families.set_len(queue_family_count as usize);}
        let mut i: u32 = 0;
        for queue_family in queue_families {
            if queue_family.queueFlags & VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT as u32 != 0 {
                indices.graphics_family = Some(i);
            }
            let mut preset_support: vk::VkBool32 = false as u32;
            let _ = unsafe{vk::vkGetPhysicalDeviceSurfaceSupportKHR(*device, i as u32, self.surface, &mut preset_support)};
            if preset_support != 0 {
                indices.preset_family = Some(i);
            }
            if indices.is_complete() {
                break;
            }
            i += 1;
        }
        return indices;
    }
    fn create_logical_device(&mut self) {
        println!("\n----- create_logical_device -----");
        let indices = self.find_queue_families(&self.physical_device);
        let mut queue_create_infos = Vec::new();
        let mut unique_queue_families = HashSet::new();
        unique_queue_families.insert(indices.graphics_family);
        unique_queue_families.insert(indices.preset_family);
        let queue_priority = 1.0f32;

        for queue_families in unique_queue_families {
            let mut queue_create_info: vk::VkDeviceQueueCreateInfo = unsafe {MaybeUninit::zeroed().assume_init()};
            queue_create_info.sType = vk::VkStructureType_VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
            queue_create_info.queueCount = 1;
            queue_create_info.pQueuePriorities = &queue_priority;
            println!("崩在这儿没问题");
            queue_create_info.queueFamilyIndex = queue_families.unwrap();
            queue_create_infos.push(queue_create_info);
        }

        let mut device_feature: vk::VkPhysicalDeviceFeatures = unsafe {MaybeUninit::zeroed().assume_init()};
        let mut create_info: vk::VkDeviceCreateInfo = unsafe {MaybeUninit::zeroed().assume_init()};
        create_info.sType = vk::VkStructureType_VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
        create_info.queueCreateInfoCount = queue_create_infos.len() as u32;
        create_info.pQueueCreateInfos = queue_create_infos.as_mut_ptr();
        create_info.pEnabledFeatures = &device_feature;
        let mut extensions = Vec::new();
        extensions.push(VK_KHR_SWAPCHAIN_EXTENSION_NAME.as_ptr() as *const c_char);
        create_info.enabledExtensionCount = extensions.len() as u32;
        create_info.ppEnabledExtensionNames = extensions.as_mut_ptr();
        if enable_validation_layers {
            create_info.enabledLayerCount = validation_layers.len() as u32;
            create_info.ppEnabledLayerNames = validation_layers.as_ptr() as *const *const c_char;
        } else {
            create_info.enabledLayerCount = 0;
        }
        if unsafe{vk::vkCreateDevice(self.physical_device, &create_info, std::ptr::null_mut(), &mut self.device)} != vk::VkResult_VK_SUCCESS {
            panic!("failed to create logical device!");
        }
        if let Some(value) = indices.graphics_family {
            unsafe{vk::vkGetDeviceQueue(self.device, value, 0, &mut self.graphics_queue);}
        }
        if let Some(value) = indices.preset_family {
            unsafe{vk::vkGetDeviceQueue(self.device, value, 0, &mut self.preset_queue);}
        }
    }
    fn create_suface(&mut self) {
        println!("\n----- create_suface -----");
        if unsafe{SDL_Vulkan_CreateSurface(self.window, self.instance as *mut rc::sdl2::VkInstance_T, &mut self.surface as *mut _ as *mut *mut rc::sdl2::VkSurfaceKHR_T)} != SDL_bool_SDL_TRUE {
            panic!("failed to create vulkan surface!");
        }
        if self.surface.is_null() {
            panic!("failed to create vulkan surface 2!");
        }
    }
    fn craete_swap_chain(&mut self) {
        println!("\n----- craete_swap_chain -----");
        let swap_chain_support = self.query_swap_chain_support(&self.physical_device);
        let surface_format: &vk::VkSurfaceFormatKHR = self.choose_swap_surface_format(&swap_chain_support.formats);
        let present_mode: &vk::VkPresentModeKHR = self.choose_swap_present_mode(&swap_chain_support.present_modes);
        let extent = self.choose_swap_extent(&swap_chain_support.capabilities);
        let mut image_count = swap_chain_support.capabilities.maxImageCount + 1;
        if swap_chain_support.capabilities.maxImageCount > 0 && image_count > swap_chain_support.capabilities.maxImageCount {
            image_count = swap_chain_support.capabilities.maxImageCount;
        }
        let mut create_info: vk::VkSwapchainCreateInfoKHR = unsafe {MaybeUninit::zeroed().assume_init()};
        create_info.sType = vk::VkStructureType_VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR;
        create_info.surface = self.surface;
        create_info.minImageCount = image_count;
        create_info.imageFormat = surface_format.format;
        create_info.imageColorSpace = surface_format.colorSpace;
        create_info.imageExtent = extent;
        create_info.imageArrayLayers = 1;
        create_info.imageUsage = vk::VkImageUsageFlagBits_VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT as u32;
        let indices: queue_family_indices = self.find_queue_families(&self.physical_device);
        let mut queue_family_indices = vec![indices.graphics_family.unwrap(), indices.preset_family.unwrap()];
        if indices.graphics_family != indices.preset_family {
            create_info.imageSharingMode = vk::VkSharingMode_VK_SHARING_MODE_CONCURRENT;
            create_info.queueFamilyIndexCount = 2;
            create_info.pQueueFamilyIndices = queue_family_indices.as_mut_ptr();
        } else {
            create_info.imageSharingMode = vk::VkSharingMode_VK_SHARING_MODE_EXCLUSIVE;
        }
        create_info.preTransform = swap_chain_support.capabilities.currentTransform;
        create_info.compositeAlpha = vk::VkCompositeAlphaFlagBitsKHR_VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR;
        create_info.presentMode = *present_mode;
        create_info.clipped = vk::VK_TRUE;
        create_info.oldSwapchain = std::ptr::null_mut();
        if unsafe{vk::vkCreateSwapchainKHR(self.device, &mut create_info, std::ptr::null_mut(), &mut self.swap_chain)} != vk::VkResult_VK_SUCCESS {
            panic!("failed to create swap chain!");
        }
        unsafe {vk::vkGetSwapchainImagesKHR(self.device, self.swap_chain, &mut image_count, std::ptr::null_mut());}
        unsafe{self.swap_chain_images.set_len(image_count as usize)};
        println!("qqq {}, {}", self.swap_chain_images.len(), image_count);
        unsafe {vk::vkGetSwapchainImagesKHR(self.device, self.swap_chain, &mut image_count, self.swap_chain_images.as_mut_ptr());}
        self.swap_chain_image_format = surface_format.format;
        self.swap_chain_extent = extent;
    }
    fn choose_swap_extent(&self, capabilities: &VkSurfaceCapabilitiesKHR) -> vk::VkExtent2D {
        if capabilities.currentExtent.width != u32::MAX {
            return capabilities.currentExtent;
        } else {
            let mut width = 0;
            let mut height = 0;
            unsafe{SDL_Vulkan_GetDrawableSize(self.window, &mut width, &mut height)};
            let mut actual_extent = unsafe{vk::VkExtent2D{width: width as u32, height: height as u32}};
            actual_extent.width = rc::clamp(actual_extent.width.clone(), capabilities.minImageExtent.width.clone(), capabilities.maxImageExtent.width.clone());
            actual_extent.height = rc::clamp(actual_extent.height.clone(), capabilities.minImageExtent.height.clone(), capabilities.maxImageExtent.height.clone());
            return actual_extent;
        }
    }
    fn choose_swap_surface_format<'a>(&self, available_formats: &'a [vk::VkSurfaceFormatKHR]) -> &'a vk::VkSurfaceFormatKHR {
        for available_format in available_formats {
            if available_format.format == vk::VkFormat_VK_FORMAT_B8G8R8A8_SRGB && available_format.colorSpace == vk::VkColorSpaceKHR_VK_COLOR_SPACE_SRGB_NONLINEAR_KHR {
                return available_format;
            }
        }
        return &available_formats[0];
    }
    fn choose_swap_present_mode<'a>(&mut self, available_present_modes: &'a Vec<vk::VkPresentModeKHR>) -> & 'a vk::VkPresentModeKHR {
        for available_present_mode in available_present_modes {
            if *available_present_mode == VkPresentModeKHR_VK_PRESENT_MODE_MAILBOX_KHR {
                return available_present_mode;
            }
        }
        return &vk::VkPresentModeKHR_VK_PRESENT_MODE_FIFO_KHR;
    }
    fn create_image_views(&mut self) {
        println!("\n----- create_image_views -----");
        unsafe{self.swap_chain_image_views.set_len(self.swap_chain_images.len());}
        for i in 0 .. self.swap_chain_images.len() {
            println!("111");
            let mut create_info: vk::VkImageViewCreateInfo = unsafe{MaybeUninit::zeroed().assume_init()};
            println!("222");
            create_info.sType = vk::VkStructureType_VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
            println!("333");
            create_info.image = self.swap_chain_images[i];
            println!("444");
            create_info.viewType = vk::VkImageViewType_VK_IMAGE_VIEW_TYPE_2D;
            create_info.format = self.swap_chain_image_format;
            create_info.components.r = vk::VkComponentSwizzle_VK_COMPONENT_SWIZZLE_IDENTITY;
            create_info.components.g = vk::VkComponentSwizzle_VK_COMPONENT_SWIZZLE_IDENTITY;
            create_info.components.b = vk::VkComponentSwizzle_VK_COMPONENT_SWIZZLE_IDENTITY;
            create_info.components.a = vk::VkComponentSwizzle_VK_COMPONENT_SWIZZLE_IDENTITY;
            create_info.subresourceRange.aspectMask = vk::VkImageAspectFlagBits_VK_IMAGE_ASPECT_COLOR_BIT as u32;
            create_info.subresourceRange.baseMipLevel = 0;
            create_info.subresourceRange.levelCount = 1;
            create_info.subresourceRange.baseArrayLayer = 0;
            create_info.subresourceRange.layerCount = 1;
            if unsafe{vk::vkCreateImageView(self.device, &create_info, std::ptr::null_mut(), &mut self.swap_chain_image_views[i])} != vk::VkResult_VK_SUCCESS {
                panic!("failed to create image views!");
            }
        }
    }
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    println!("Hello, world!");
    let mut app = app::new();
    app.run();
}

unsafe extern "C" fn debug_callback(message_severity: vk::VkDebugUtilsMessageSeverityFlagBitsEXT, message_type: u32, p_callback_data: *const vk::VkDebugUtilsMessengerCallbackDataEXT, user_data: *mut std::ffi::c_void) -> vk::VkBool32 {       
    let message = unsafe {
        let c_msg = (*p_callback_data).pMessage;
        if c_msg.is_null() {
            "<null>".to_string()
        } else {
            CStr::from_ptr(c_msg).to_string_lossy().into_owned()
        }
    };
    println!("***ValidationLayer: {}", message);
    vk::VK_FALSE
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