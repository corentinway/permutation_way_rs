
extern crate permutation_way;

use permutation_way::PermutationIterator;

use std::thread;
use std::sync::mpsc;
use std::marker::Send;


fn main() {

    let vec_length = get_vec_length();

    println!( "Vec length is {}", vec_length );

    let input = create_input_vector(vec_length);

    println!( "input is {:?}", input );

    find_permutations( input, |data| {
        println!("callback - {:?}", data );
    }  );

    thread::sleep_ms( 10000 );

}


fn find_permutations<F>( input : Vec<i32>, callback : F ) 
    where F : 'static + Fn(Vec<i32>) + Send  {

    thread::spawn(move || {
       let iter = PermutationIterator::new( input );

        for permutation in iter {
            callback( permutation );
        }
    });

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