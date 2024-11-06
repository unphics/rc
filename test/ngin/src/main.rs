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

use ash::extensions::ext::DebugUtils;

const Width: i32 = 800;
const Height: i32 = 600;
const validation_layers: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];
const enable_validation_layers: bool = true;

struct app {
    window: *mut SDL_Window,
    instance: vk::VkInstance,
    debug_messenger: vk::VkDebugUtilsMessengerEXT,
}
impl app {
    pub fn new() -> Self {
        Self {
            window: std::ptr::null_mut(),
            instance: std::ptr::null_mut(),
            debug_messenger: std::ptr::null_mut(),
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
    }
    fn main_loop(&self) {
        std::thread::sleep(std::time::Duration::from_secs(4));
    }
    fn clean_up(&self) {
        unsafe {
            if enable_validation_layers {
                // vk::vkDestroyDebugUtilsMessengerEXT(self.instance, self.debug_messenger, std::ptr::null());
            }
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
        app_info.applicationVersion = vk::VK_VERSION_1_0;
        app_info.pEngineName = "No Engine".as_ptr() as *const c_char;
        app_info.engineVersion = vk::VK_VERSION_1_0;
        app_info.apiVersion = vk::VK_VERSION_1_0;

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
    }
    fn get_required_extensions(&self) -> Vec<*const c_char> {
        let mut extensions: Vec<*const c_char> = Vec::new();
        extensions.push(VK_EXT_DEBUG_UTILS_EXTENSION_NAME.as_ptr() as *const c_char);
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
        let func: vk::PFN_vkVoidFunction = unsafe{vk::vkGetInstanceProcAddr(self.instance, "vkCreateDebugUtilsMessengerEXT".as_ptr() as *const c_char)};
        unsafe {
            println!("111");
            std::thread::sleep(std::time::Duration::from_secs(1));
            let p: *mut std::ffi::c_void = std::mem::transmute(func);
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("222{:p}", p);
        }
        let f: vk::PFN_vkCreateDebugUtilsMessengerEXT = unsafe{std::mem::transmute(func)};
        if let Some(foo) = f {
            unsafe{foo(self.instance, create_info, std::ptr::null(), &mut self.debug_messenger)};
        } else {
            panic!("vkCreateDebugUtilsMessengerEXT not found");
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
}

fn main() {
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
    println!("Debug callback: {}", message);
    vk::VK_FALSE
}
