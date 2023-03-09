//! This is an implementation of the [Steinhaus Johnson Trotter algorithm with Even's speedup](https://en.wikipedia.org/wiki/Steinhaus%E2%80%93Johnson%E2%80%93Trotter_algorithm#Even's_speedup).
//! This algorithm compute all permutations of a given input vector.

use crate::mobility::create_directions;
use crate::mobility::Mobility;
use crate::mobility::Mobility::*;
use crate::mobility::MobilityError::*;

use crate::utils::find_largest_mobile_element;

/// An iterator to find all permutations of the given `input` vector.
///
/// # Example
///
/// ```
/// use permutation_way::PermutationIterator;
///   // input
///   let input = vec![1, 2, 3];
///   // call
///   let mut iterator = PermutationIterator::new( input );
///   // assertions
///   assert_eq!( Some( vec![1, 2, 3] ), iterator.next() );
///   assert_eq!( Some( vec![1, 3, 2] ), iterator.next() );
///   assert_eq!( Some( vec![3, 1, 2] ), iterator.next() );
///   assert_eq!( Some( vec![3, 2, 1] ), iterator.next() );
///   assert_eq!( Some( vec![2, 3, 1] ), iterator.next() );
///   assert_eq!( Some( vec![2, 1, 3] ), iterator.next() );
///   assert_eq!( None, iterator.next() );
/// ```
pub struct PermutationIterator<T> {
    /// vec that should permute its elements
    input: Vec<T>,
    /// vec that holds mobility information of each element of the `input`array
    directions: Vec<Mobility>,
    /// internal counter useful to emit the 1st array
    counter: u32,
    /// result to check if some errors happened
    result: Result<(), String>,
    /// maximum expected permutation. The iterator should return none if the maximum is reached
    max: Option<u32>,
}

impl<T> PermutationIterator<T>
where
    T: PartialOrd + Ord,
{
    /// Return an instance of an `Iterator` that will provide
    /// each permutation when `next` method is invoked
    pub fn new(input: Vec<T>) -> PermutationIterator<T> {
        let directions = create_directions(&input);

        let mut iterator = PermutationIterator {
            input,
            directions,
            counter: 0,
            result: Ok(()),
            max: None,
        };

        iterator.input.sort();

        iterator
    }

    /// Return `false` if all permutations occurs well
    pub fn has_errors(&self) -> bool {
        return self.result.is_err();
    }

    /// set the maximum permutation to comput
    /// e
    /// # Example
    ///
    /// ```
    /// use permutation_way::PermutationIterator;
    ///   // input
    ///   let input = vec![1, 2, 3];
    ///   // call
    ///   let mut iterator = PermutationIterator::new( input );
    ///   iterator.set_max( 3 );
    ///   // assertions
    ///   assert_eq!( Some( vec![1, 2, 3] ), iterator.next() );
    ///   assert_eq!( Some( vec![1, 3, 2] ), iterator.next() );
    ///   assert_eq!( Some( vec![3, 1, 2] ), iterator.next() );
    ///   assert_eq!( None, iterator.next() );
    ///   assert_eq!( false, iterator.has_errors() );
    /// ```
    pub fn set_max(&mut self, max: u32) {
        self.max = Some(max);
    }
}

impl<T> Iterator for PermutationIterator<T>
where
    T: PartialOrd + Clone,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.len() == 0 {
            return None;
        }

        // check max is reach
        if self.max == Some(self.counter) {
            return None;
        }

        if self.counter == 0 {
            self.counter = self.counter + 1;
            return Some(self.input.clone());
        } else if self.input.len() == 1 {
            // stop
            return None;
        }

        let largest = find_largest_mobile_element(&self.input, &self.directions);
        let direction = largest.direction;
        let mobile_position = largest.position;

        if direction == NotMobile {
            self.counter = self.counter + 1;
            return None;
        }

        let swap_result = direction.swap(&mut self.input, &mut self.directions, mobile_position);

        if let Err(SwapError(position)) = swap_result {
            self.result = Err(format!("swap permutation error at position {}", position));
            // stop the iterator
            return None;
        }
        let result = Some(self.input.clone());

        let reset_result = direction.reset(&self.input, &mut self.directions, mobile_position);

        if let Err(SwapError(position)) = reset_result {
            self.result = Err(format!("swap permutation error at position {}", position));
            // stop the iterator
            return None;
        }
        if let Err(ResetError(position)) = reset_result {
            self.result = Err(format!("reset permutation error at position {}", position));
            // stop the iterator
            return None;
        }

        self.counter = self.counter + 1;
        result
    }
}

