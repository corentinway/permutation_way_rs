extern crate permutation_way;

use permutation_way::compute_permutation;

#[test]
fn should_return_an_empty_vec_given_an_empty_array() {
    // input
    let input: Vec<i32> = vec![];

    // call
    let mut iterator = compute_permutation(input);

    // assertions
    assert_eq!(None, iterator.next());
    assert_eq!(false, iterator.has_errors());
}

#[test]
fn should_return_a_maximum_of_3_permutations_given_a_vec_of_3_elements_and_a_threshold_of_3() {
    // input
    let input = vec![1, 2, 3];
    // call
    let mut iterator = compute_permutation(input);
    iterator.set_max(3);
    // assertions
    assert_eq!(Some(vec![1, 2, 3]), iterator.next());
    assert_eq!(Some(vec![1, 3, 2]), iterator.next());
    assert_eq!(Some(vec![3, 1, 2]), iterator.next());
    assert_eq!(None, iterator.next());
    assert_eq!(false, iterator.has_errors());
}

#[test]
fn should_find_6_permutation_given_a_vec_of_3_elements_with_iterator() {
    // input
    let input = vec![1, 2, 3];
    // call
    let mut iterator = compute_permutation(input);
    // assertions
    assert_eq!(Some(vec![1, 2, 3]), iterator.next());
    assert_eq!(Some(vec![1, 3, 2]), iterator.next());
    assert_eq!(Some(vec![3, 1, 2]), iterator.next());
    assert_eq!(Some(vec![3, 2, 1]), iterator.next());
    assert_eq!(Some(vec![2, 3, 1]), iterator.next());
    assert_eq!(Some(vec![2, 1, 3]), iterator.next());
    assert_eq!(None, iterator.next());
    assert_eq!(false, iterator.has_errors());
}

#[test]
fn should_find_1_permutation_given_a_vec_of_1_elements_with_iterator() {
    // input
    let input = vec![1];
    // call
    let mut iterator = compute_permutation(input);
    // assertions
    assert_eq!(Some(vec![1]), iterator.next());
    assert_eq!(None, iterator.next());
    assert_eq!(false, iterator.has_errors());
}

#[test]
fn should_find_2_permutation_given_a_vec_of_2_elements_with_iterator() {
    // input
    let input = vec![1, 2];
    // call
    let mut iterator = compute_permutation(input);
    // assertions
    assert_eq!(Some(vec![1, 2]), iterator.next());
    assert_eq!(Some(vec![2, 1]), iterator.next());
    assert_eq!(None, iterator.next());
    assert_eq!(false, iterator.has_errors());
}
