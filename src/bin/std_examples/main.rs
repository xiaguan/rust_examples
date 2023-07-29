#![feature(ptr_internals)]

use std::ptr::Unique;

#[allow(dead_code)]
struct MyVec {
    data: Unique<i32>,
    alloc: Unique<i64>,
    cap: usize,
    len: usize,
}

fn main() {
    use std::any::{Any, TypeId};

    let boxed: Box<dyn Any> = Box::new(3_i32);

    let my_vec = MyVec {
        data: unsafe { Unique::new_unchecked(0x2222 as *mut i32) },
        alloc: unsafe { Unique::new_unchecked(0x3333 as *mut i64) },
        cap: 0x11111111,
        len: 0xffffffff,
    };
    // hh

    // use unsafe to print the my_vec's binary representation
    unsafe {
        let my_vec_ptr = &my_vec as *const MyVec;
        let my_vec_ptr = my_vec_ptr as *const u8;
        let my_vec_ptr = my_vec_ptr as *const [u8; std::mem::size_of::<MyVec>()];
        let my_vec_ptr = &*my_vec_ptr;
        println!("my_vec: {:?}", my_vec_ptr);
    }

    println!("size of Vec<i32>: {}", std::mem::size_of::<Vec<i32>>());

    // You're more likely to want this:
    let actual_id = (&*boxed).type_id();
    // ... than this:
    let boxed_id = boxed.type_id();

    assert_eq!(actual_id, TypeId::of::<i32>());
    assert_eq!(boxed_id, TypeId::of::<Box<dyn Any>>());
}
