
extern crate permutation_way;

use permutation_way::PermutationIterator;

use std::time::Instant;

fn main() {



    let input = vec![1, 2, 3];


    let start = Instant::now();

    // call
    let iterator = PermutationIterator::new( input );

    for _val in iterator {
        // do nothing
    }

    let end = Instant::now();


    println!("Execution time = {:?}", end.duration_since(start));


    // input
    let input = vec![1, 2, 3];
    // call
    let iterator = PermutationIterator::new( input );

    for val in iterator {
        println!( " value = {:?}", val );
    }




}
