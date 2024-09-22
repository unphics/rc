use rc::malloc;
use rc::free;
use rc::deref;

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

fn main() {
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
}