// simd.rs
#![feature(portable_simd)]
use std::simd::f32x4;

fn main() {
    // create simd vectors
    let x = f32x4::from([1.0, 2.0, 3.0, 4.0]);
    let y = f32x4::from([4.0, 3.0, 2.0, 1.0]);

    // simd product
    let z = x * y;

    // like any struct, the simd vector can be destructured using `let`
    let result_array = z.as_array();

    println!("{:?}", result_array);
}
