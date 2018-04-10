
extern crate permutation_way;

use permutation_way::permute;
use permutation_way::PermutationIterator;

use std::time::Instant;

fn main() {



    let input = vec![1, 2, 3];


    let start = Instant::now();

    let permutations = permute( input );

    let end = Instant::now();


    println!("Execution time = {:?}", end.duration_since(start));

    match permutations {
        Ok(permutations) => println!("Permutation found {}", permutations.len() ),
        Err(message) => println!("Found exception of mobility saying: {}", message),
    }

    // input
    let input = vec![1, 2, 3];
    // call
    let mut iterator = PermutationIterator::new( input );

    for val in iterator {
        println!( " value = {:?}", val );
    }




}
