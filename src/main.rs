
extern crate permutation_way;

use permutation_way::PermutationIterator;

use std::thread;
use std::sync::mpsc;


fn main() {



    let (transmitter, receiver) = mpsc::channel();

    // thread that compute permutations;
    let compute_handler = thread::spawn( move || {

        // input
        let input = vec![1, 2, 3];
        // iterator
        let iterator = PermutationIterator::new( input );

        iterator.enumerate().for_each( move|permutation| 
            // println!( "found {:?}", permutation );
            transmitter.send( permutation ).unwrap()
        );
        
    });


    for permutation in receiver.iter() {
        println!( "Got {:?}", permutation );
    }


    compute_handler.join().unwrap();




}
