use proc_macro_rc::mytest_proc_macro;
use proc_macro_rc::Builder;

// #[mytest_proc_macro(qwer)]
// pub fn foo(id: i32) {}

#[derive(Builder)]
pub struct Command {
    executable:String,
}

fn main() {
    let mut a = Command::builder();
    a.executable = Some("qqqq".to_string());
    let b = a.executable.unwrap();
    println!("derive command b = {}", b.as_str());
}