use rc::malloc;
use rc::free;
use rc::deref;
use std::time::Duration;

struct A {
    id: u32,
}

static mut COUNT: u32 = 0;

impl A {
    pub fn new() -> Self {
        unsafe {
            COUNT += 1;
        }
        return  A {
            id: 1,
        }
    }
    pub fn add(&mut self) {
        unsafe {
            COUNT += 2;
        }
        self.id += 1;
    }
}
impl Drop for A {
    fn drop(&mut self) {
        unsafe {
            COUNT = if COUNT == 3 {0} else {1}
        }
    }
}

struct B {
    arr: [u64; 10000]
}
impl B {
    pub fn new() -> Self {
        return B{arr:[123; 10000]}
    }
}

fn main() {
    let a = malloc(A::new());
    deref(a).add();
    free(a);
    for _ in 0..1000 {
        let mut a = A::new();
        a.add();
    }
    unsafe {
        println!("COUNT = {}", COUNT);
    }
    for _ in 0..1000 {
        let a = malloc(A::new());
        deref(a).add();
        free(a);
    }
    unsafe {
        println!("COUNT = {}", COUNT);
    }
    println!("001");
    let mut vec = Vec::new();
    for _ in 0..1000 {
        let p = malloc(B::new());
        vec.push(p);
    }
    std::thread::sleep(Duration::from_secs(5));
    println!("002");
    for p in vec {
        free(p);
    }
    std::thread::sleep(Duration::from_secs(5));
    println!("003");
}