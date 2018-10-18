extern crate permutation_way;

use permutation_way::PermutationIteratorThread;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

use std::time::Duration;

#[test]
fn should_be_coded_with_threads() {
    let input = vec![1, 2, 3];

    let receiver = |permutation| {
        println!("permutation received {:?}", permutation);
    };

    // call
    PermutationIteratorThread::new(input, receiver);

    thread::sleep(Duration::from_secs(5));

    // assertions
}
#[test]
fn should_invoke_a_mutable_callback() {
    let input = vec![1, 2, 3];
    let mut counter = 0;
    let receiver = |permutation| {
        counter = counter + 1;
    };
    // call
    PermutationIteratorThread::iter_mut( input, receiver );
    thread::sleep(Duration::from_secs(10));
    // assertions
    assert_eq!( 6, counter );

}
