extern crate permutation_way;

use permutation_way::PermutationIterator;

#[test]
fn should_return_an_empty_vec_given_an_empty_array() {

    // input
    let input = vec![];

    // call
    let mut iterator = PermutationIterator::new( input );

    // assertions
    assert_eq!( None, iterator.next() );

}

#[test]
fn should_find_6_permutation_given_a_vec_of_3_elements_with_iterator() {
    // input
    let input = vec![1, 2, 3];
    // call
    let mut iterator = PermutationIterator::new( input );
    // assertions
    assert_eq!( Some( vec![1, 2, 3] ), iterator.next() );
    assert_eq!( Some( vec![1, 3, 2] ), iterator.next() );
    assert_eq!( Some( vec![3, 1, 2] ), iterator.next() );
    assert_eq!( Some( vec![3, 2, 1] ), iterator.next() );
    assert_eq!( Some( vec![2, 3, 1] ), iterator.next() );
    assert_eq!( Some( vec![2, 1, 3] ), iterator.next() );
    assert_eq!( None, iterator.next() );
}
