use rc::*;

fn main() {
    unsafe {
        let ctx = ImGui_CreateContext(std::ptr::null_mut());
        if ctx.is_null() {
            println!("failed to create imgui context");
        } else {
            println!("succeed to create imgui context");
        }
    }
}