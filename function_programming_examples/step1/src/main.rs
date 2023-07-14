use std::sync::Arc;

fn print_multiplication_table() {
    for i in 1..10 {
        for j in 1..=i {
            print!("| {i} * {j}={:2}|", i * j);
        }
        println!();
    }

    println!("");
    // In fact,it's a three elements tuple(i,j,i*j)
    // we use function programming style to print it
    (0..10)
        .flat_map(|i| (1..=i).map(move |j| (i, j, i * j)))
        .for_each(|(i, j, product)| {
            print!("| {i} * {j}={product:2}|");
            if i == j {
                println!();
            }
        });
}

fn map_and_flat() {
    let words = ["alpha", "beta", "gamma"];

    // chars() returns an iterator
    let merged = words.iter().map(|s| s.chars()).flatten();
    println!("{:?}", merged.collect::<String>());
}

fn into_iterators() {
    let result = (0..10) // 0,1,...9
        .map(|x| x * x) // 0,1,...81
        .inspect(|x| println!("value {}", *x))
        .filter(|x| x < &20)
        .inspect(|x| println!("value pass {}", *x))
        .fold(0, |x, y| x + y);
    println!("result {}", result);
}

fn main() {
    into_iterators();
    map_and_flat();
    print_multiplication_table();
}
