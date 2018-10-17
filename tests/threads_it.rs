extern crate permutation_way;

use permutation_way::PermutationIteratorThread;

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;


use std::time::Duration;

#[test]
fn should_be_coded_with_threads() {


    let input = vec![1, 2, 3];

    let receiver = | permutation | {
        println!( "permutation received {:?}", permutation );
    };

    // call
    PermutationIteratorThread::new( input, receiver );

    thread::sleep(Duration::from_secs(5));

    // assertions
    

}