use core::slice;

fn dereference_raw_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is :{}", *r1);
        println!("r2 is: {}", *r2);
    }

    let address = 0x012345usize;
    let r = address as *const i32;
}

fn calling_unsafe_function_or_method() {
    unsafe fn danger() {}

    unsafe {
        danger();
    }
}

fn safe_abstraction_over_unsafe_code() {
    // fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //     let len = slice.len();
    //     assert!(mid <= len);
    //     (&mut slice[..mid], &mut slice[mid..])
    // }

    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        assert!(mid <= len);

        let ptr = slice.as_mut_ptr();
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len),
            )
        }
    }
}

fn extern_functions_to_call_external_code() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe { println!("Absolute value of -3 according to C: {}", abs(-3)) }

    // // compiler's name mangling
    // #[no_mangle]
    // pub extern "C" fn call_from_c() {
    //     println!("Just called a Rust function from C!")
    // }
}

fn accessing_or_modifying_mutable_static_variable() {
    static mut HELLO: u32 = 0;

    unsafe {
        HELLO += 1;
    }

    unsafe {
        println!("name is: {}", HELLO);
    }
}

fn implementing_unsafe_trait() {
    unsafe trait Foo {}

    unsafe impl Foo for i32 {}
}

fn accessing_fields_of_union() {}
