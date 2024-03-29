extern crate permutation_way;

use permutation_way::compute_permutation;

use std::time::Instant;

use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let vec_length = if arguments.len() > 1 {
        arguments.get(1).unwrap().parse().unwrap()
    } else {
        3
    };

    println!("Vec length is {}", vec_length);

    let mut input: Vec<i32> = Vec::with_capacity(vec_length);
    for data in 0..vec_length {
        input.push(data as i32);
    }

    println!("input is {:?}", input);

    let start = Instant::now();

    // call
    let iterator = compute_permutation(input);

    let permutation_found_counter = iterator.count();

    let end = Instant::now();

    println!(
        "Found {} permutation in {:?}",
        permutation_found_counter,
        end.duration_since(start)
    );
}
