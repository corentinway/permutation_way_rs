extern crate permutation_way;

use permutation_way::PermutationIteratorThread;
use permutation_way::PermutationIteratorEvent;
use permutation_way::PermutationIteratorEvent::*;

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;


use std::time::Duration;

#[test]
fn should_be_coded_with_threads() {

    // counter of permutation received
    let mut counter = 0;

    // let (tx, rx) : (Sender<PermutationIteratorEvent<i32>>, Receiver<PermutationIteratorEvent<i32>>) 
    //     = mpsc::channel();


    let input = vec![1, 2, 3];

    let receiver = move | event | {
        println!( "evenement reçus {:?}", event );
        match event {
            Data( index, permutation) => {
                match index {
                    0 => assert_eq!( vec![1, 2, 3], permutation ),
                    1 => assert_eq!( vec![1, 3, 2], permutation ),
                    2 => assert_eq!( vec![3, 1, 2], permutation ),
                    3 => assert_eq!( vec![3, 2, 1], permutation ),
                    4 => assert_eq!( vec![2, 3, 1], permutation ),
                    5 => assert_eq!( vec![2, 1, 3], permutation ),
                    // 1 -> assert_eq!( None, iterator.next() ),
                    // 1 -> assert_eq!( false, iterator.has_errors() ),
                    _ => (),
                }
                counter = counter + 1
            },
            End => {
                assert_eq!( 6, counter )
                  
                //tx.send( event ).unwrap()
            }

        }
    };


    // call
    PermutationIteratorThread::new( input, & mut receiver );

    // assertions
    // wait receiving the end event
    //let result = rx.recv_timeout( Duration::from_secs(10) );
    //assert_eq!( End, result.unwrap() );
    
}