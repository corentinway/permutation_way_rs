
extern crate permutation_way;

use permutation_way::PermutationIterator;

//use std::time::Instant;

use std::thread;
use std::sync::mpsc;


fn main() {

    let vec_length = get_vec_length();

    println!( "Vec length is {}", vec_length );

    let input = create_input_vector(vec_length);

    println!( "input is {:?}", input );

    find_permutations( input, |data| {
        println!("callback - {:?}", data );
    }  );

}


fn find_permutations<F>( input : Vec<i32>, callback : F ) 
    where F : Fn(Vec<i32>)  {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
       let iter = PermutationIterator::new( input );

        for permutation in iter {
            tx.send( permutation ).unwrap();
        }
    });

    let iter = rx.iter();
    let mut received_count = 0;
    for received in iter {
        received_count = received_count + 1;
        callback( received );
    }

}

fn create_input_vector( vec_length : usize ) -> Vec<i32> {
    let mut input : Vec<i32> = Vec::with_capacity( vec_length );
    for data in 0..vec_length {
        input.push( data as i32 );
    }

    input
}

fn get_vec_length() -> usize {

    use std::env;

    let arguments : Vec<String> = env::args()
    .collect();

    if arguments.len() > 1 {
        arguments.get(1).unwrap().parse().unwrap()
    } else {
        3
    }
}