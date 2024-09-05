pub fn malloc<T>(obj: T) -> *mut T where T: Sized {
    let layout = std::alloc::Layout::new::<T>();
    let mut ptr_deap : *mut T = std::ptr::null_mut();
    unsafe {
        ptr_deap = std::alloc::alloc(layout) as *mut T;
        ptr_deap.write(obj);
    }
    return ptr_deap;
}

pub fn free<T>(ptr_deap: *mut T) {
    let layout = std::alloc::Layout::new::<T>();
    unsafe {
        std::alloc::dealloc(ptr_deap as *mut u8, layout);
    }
}

pub fn deref<'a, T>(ptr: *mut T) -> &'a mut T  where T: Sized {
    return unsafe{&mut*ptr};
}

#[cfg(test)]
mod tests {
    use crate::malloc;
    use crate::deref;
    use crate::free;
    pub struct A {
        id: u32,
        name: String,
    }
    impl A {
        pub fn new() ->Self {
            return A {id:12, name: "a".to_string()};
        }
    }
    #[test]
    fn alloc_work_test() {
        println!("alloc_work_test");
        let mut a: *mut A = std::ptr::null_mut();
        println!("a = {:p}", a);
        {
            a = malloc(A::new());
            println!("a = {:p}, a.id = {}", a, deref(a).id);
        }
        println!("a = {:p}, a.id = {}", a, deref(a).id);
        free(a);
        println!("a = {:p}, a.id = {}", a, deref(a).id); // undefined behaviour
    }
}