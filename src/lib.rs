//! This is an implementation of the [Steinhaus Johnson Trotter algorithm with Even's speedup](https://en.wikipedia.org/wiki/Steinhaus%E2%80%93Johnson%E2%80%93Trotter_algorithm#Even's_speedup).
//! This algorithm compute all permutations of a given input vector.

mod mobility;
mod permutation_iterator;
mod utils;

use permutation_iterator::PermutationIterator;

pub fn compute_permutation<T>(input: Vec<T>) -> PermutationIterator<T>
where
    T: PartialOrd + Ord + Clone,
{
    PermutationIterator::new(input)
}
