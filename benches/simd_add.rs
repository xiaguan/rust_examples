// simd_add.rs
#![feature(portable_simd)]

use std::simd::f32x4;
use std::simd::f32x8;

use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

macro_rules! assert_equal_len {
    ($a:ident, $b: ident) => {
        assert!(
            $a.len() == $b.len(),
            "add_assign: dimension mismatch: {:?} += {:?}",
            ($a.len(),),
            ($b.len(),)
        );
    };
}

// element-wise addition
fn add_assign(xs: &mut Vec<f32>, ys: &Vec<f32>) {
    assert_equal_len!(xs, ys);

    for (x, y) in xs.iter_mut().zip(ys.iter()) {
        *x += *y;
    }
}

// simd accelerated addition
fn simd_add_assign(xs: &mut Vec<f32>, ys: &Vec<f32>) {
    assert_equal_len!(xs, ys);

    let size = xs.len() as isize;
    let chunks = size / 8;

    // pointer to the start of the vector data
    let p_x: *mut f32 = xs.as_mut_ptr();
    let p_y: *const f32 = ys.as_ptr();

    // sum excess elements that don't fit in the simd vector
    for i in (8 * chunks)..size {
        // dereferencing a raw pointer requires an unsafe block
        unsafe {
            // offset by i elements
            *p_x.offset(i) += *p_y.offset(i);
        }
    }

    // treat f32 vector as an simd f32x4 vector
    let simd_p_x = p_x as *mut f32x8;
    let simd_p_y = p_y as *const f32x8;

    // sum "simd vector"
    for i in 0..chunks {
        unsafe {
            *simd_p_x.offset(i) += *simd_p_y.offset(i);
        }
    }
}

// generate a vector of random floats
fn generate_random_vector(size: usize) -> Vec<f32> {
    let mut vec = Vec::with_capacity(size);
    for i in 0..size {
        vec.push(i as f32);
    }
    vec
}
const TEST_SIZE: usize = 8 * 1024 * 1024;
// benchmark
fn bench_add_assign(c: &mut Criterion) {
    let mut xs = generate_random_vector(TEST_SIZE);
    let ys = generate_random_vector(TEST_SIZE);

    // use black_box to prevent compiler from optimizing out the loop
    c.bench_function("add_assign", |b| {
        b.iter(|| {
            black_box(add_assign(&mut xs, &ys));
        })
    });
}

fn bench_simd_add_assign(c: &mut Criterion) {
    let mut xs = generate_random_vector(TEST_SIZE);
    let ys = generate_random_vector(TEST_SIZE);

    // use black_box to prevent compiler from optimizing out the loop
    c.bench_function("simd_add_assign", |b| {
        b.iter(|| {
            black_box(simd_add_assign(&mut xs, &ys));
        })
    });
}

criterion_group!(benches, bench_add_assign, bench_simd_add_assign);
criterion_main!(benches);
