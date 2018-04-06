extern crate permutation_way;

use permutation_way::permute;
use permutation_way::PermutationEngine;

#[test]
fn should_find_6_uniq_permutation_given_a_vec_of_3_elements () {

    // input
    let input = vec![1, 2, 3];

    // call
    let permutations = permute( input );

    // assertions
    assert_eq!( Ok(vec![
        vec![1, 2, 3],
        vec![1, 3, 2],
        vec![3, 1, 2],
        vec![3, 2, 1],
        vec![2, 3, 1],
        vec![2, 1, 3]
    ]), permutations );

}
#[test]
fn should_return_an_empty_vec_given_an_empty_array() {

    // input
    let input = vec![];

    // call
    let permutations = permute( input );

    // assertions
    assert_eq!( Err( String::from("Cannot find permutation on empty vec") ), permutations );

}

#[test]
fn should_find_6_permutation_given_a_vec_of_3_elements_with_iterator() {
    // input
    let input = vec![1, 2, 3];
    // call
    let mut engine = PermutationEngine::new( input );
    // assertions
    assert_eq!( Some( vec![1, 2, 3] ), engine.next() );
    assert_eq!( Some( vec![1, 3, 2] ), engine.next() );
    assert_eq!( Some( vec![3, 1, 2] ), engine.next() );
    assert_eq!( Some( vec![3, 2, 1] ), engine.next() );
    assert_eq!( Some( vec![2, 3, 1] ), engine.next() );
    assert_eq!( Some( vec![2, 1, 3] ), engine.next() );
    assert_eq!( None, engine.next() );
}


#[test]
fn try_with_loop() {
    // input
    let input = vec![1, 2, 3];
    // call
    let mut engine = PermutationEngine::new( input );

    engine.for_each( move |permutation| println!( " permutation {:?}", permutation ));
}