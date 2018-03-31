extern crate permutation_way;

use permutation_way::permute;

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