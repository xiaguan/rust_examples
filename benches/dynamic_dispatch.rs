use criterion::{black_box, criterion_group, criterion_main, Criterion};

trait ShapeTrait {
    fn length(&self) -> f64;
    fn height(&self) -> f64;
}

struct Rectangle {
    pub length: f64,
    pub height: f64,
}

impl ShapeTrait for Rectangle {
    fn length(&self) -> f64 {
        self.length
    }

    fn height(&self) -> f64 {
        self.height
    }
}

struct Circle {
    pub radius: f64,
}

impl ShapeTrait for Circle {
    fn length(&self) -> f64 {
        self.radius * 2.0
    }

    fn height(&self) -> f64 {
        self.radius * 2.0
    }
}

#[enum_dispatch::enum_dispatch]
trait ShapeEnum {
    fn length_by_enum(&self) -> f64;
    fn height_by_enum(&self) -> f64;
}

impl ShapeEnum for Rectangle {
    fn length_by_enum(&self) -> f64 {
        self.length
    }

    fn height_by_enum(&self) -> f64 {
        self.height
    }
}

impl ShapeEnum for Circle {
    fn length_by_enum(&self) -> f64 {
        self.radius * 2.0
    }

    fn height_by_enum(&self) -> f64 {
        self.radius * 2.0
    }
}

/// Just like
/// ```
/// fn area_by_enum(&self) -> f64 {
///    match self {
///       ShapeEnumImpl::Rectangle(r) => r.area_by_enum(),
///      ShapeEnumImpl::Circle(c) => c.area_by_enum(),
/// }
/// ```
#[enum_dispatch::enum_dispatch(ShapeEnum)]
enum ShapeEnumImpl {
    Rectangle(Rectangle),
    Circle(Circle),
}

// For bench, we need to use the same trait for both enum and struct
// We generate a big vector of shapes and then call
// the same method on them
fn generate_shape_trait(size: usize) -> Vec<Box<dyn ShapeTrait>> {
    let mut shapes = Vec::with_capacity(size);
    for i in 0..size {
        if i % 2 == 0 {
            shapes.push(Box::new(Rectangle {
                length: i as f64,
                height: i as f64,
            }) as Box<dyn ShapeTrait>);
        } else {
            shapes.push(Box::new(Circle { radius: i as f64 }) as Box<dyn ShapeTrait>);
        }
    }
    shapes
}

fn generate_shape_enum(size: usize) -> Vec<ShapeEnumImpl> {
    let mut shapes = Vec::with_capacity(size);
    for i in 0..size {
        if i % 2 == 0 {
            shapes.push(ShapeEnumImpl::Rectangle(Rectangle {
                length: i as f64,
                height: i as f64,
            }));
        } else {
            shapes.push(ShapeEnumImpl::Circle(Circle { radius: i as f64 }));
        }
    }
    shapes
}

fn bench_trait(c: &mut Criterion) {
    let shapes = generate_shape_trait(100_000);
    // use black_box to prevent compiler from optimizing out the loop
    c.bench_function("trait", |b| {
        b.iter(|| {
            black_box({
                let mut total_length = 0.0;
                let mut total_height = 0.0;
                for shape in &shapes {
                    total_length += shape.length();
                    total_height += shape.height();
                }
                (total_length, total_height)
            })
        })
    });
}

fn bench_enum(c: &mut Criterion) {
    let shapes = generate_shape_enum(100_000);
    // use black_box to prevent compiler from optimizing out the loop
    c.bench_function("enum", |b| {
        b.iter(|| {
            black_box({
                let mut total_length = 0.0;
                let mut total_height = 0.0;
                for shape in &shapes {
                    total_length += shape.length_by_enum();
                    total_height += shape.height_by_enum();
                }
                (total_length, total_height)
            })
        })
    });
}

criterion_group!(benches, bench_trait, bench_enum);
criterion_main!(benches);
