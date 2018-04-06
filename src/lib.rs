//! This is an implementation of the [Steinhaus Johnson Trotter algorithm with Even's speedup](https://en.wikipedia.org/wiki/Steinhaus%E2%80%93Johnson%E2%80%93Trotter_algorithm#Even's_speedup).
//! This algorithm compute all permutations of a given input vector.

mod mobility;
mod utils;

use mobility::Mobility;
use mobility::Mobility::*;
use mobility::MobilityError::*;
use mobility::create_directions;

use utils::print_permutation;
use utils::find_largest_mobile_element;


pub struct PermutationEngine {
    input: Vec<i32>,
    directions: Vec<Mobility>,
    counter: u32,
}

impl PermutationEngine {
    pub fn new( input : Vec<i32> ) -> PermutationEngine {

        let directions = create_directions( &input );

        let mut engine = PermutationEngine{ input,
            directions,
            counter: 0 };

        engine.input.sort();

        engine
    }
}

impl Iterator for PermutationEngine {
    type Item = Vec<i32>;

    fn next( &mut self ) -> Option<Self::Item> {


        if self.counter == 0 {
            println!( "         COUNTER 0" );
            self.counter = self.counter + 1;
            return Some( self.input.clone() );
        }

        let largest = find_largest_mobile_element(&self.input, &self.directions);
        let direction = largest.direction;
        let mobile_position = largest.position;

        if direction == NotMobile {
            self.counter = self.counter + 1;
            return None;
        }

        let swap_result = direction.swap( &mut self.input, &mut self.directions, mobile_position );

        /*if let Err( SwapError(position)) = swap_result {
            return Err(format!("swap permutation error at position {}", position));
        }*/
        let result = Some( self.input.clone() );


        let reset_result = direction.reset( &self.input, &mut self.directions, mobile_position );

        /*if let Err(SwapError(position)) = reset_result {
            return Err(format!("swap permutation error at position {}", position));
        }
        if let Err(ResetError(position)) = reset_result {
            return Err(format!( "reset permutation error at position {}", position));
        }
        */
        //print_permutation( &input, &directions );


        self.counter = self.counter + 1;
        result


    }
}



/// find all permutations of the given `input` vector.
///
/// # Example
///
/// ```
/// use permutation_way::permute;
/// // input
/// let input = vec![1, 2, 3];
///
/// // call
/// let permutations = permute( input );
///
/// // assertions
/// assert_eq!( Ok(vec![
///     vec![1, 2, 3],
///     vec![1, 3, 2],
///     vec![3, 1, 2],
///     vec![3, 2, 1],
///     vec![2, 3, 1],
///     vec![2, 1, 3]
/// ]), permutations );
/// ```
pub fn permute( input : Vec<i32> ) -> Result<Vec<Vec<i32>>, String> {

    if input.is_empty() {
        return Err( String::from("Cannot find permutation on empty vec") );
    }

    let mut res = Vec::new();

    let mut input = input;
    input.sort();

    let mut directions = create_directions( &input );

    // push 1, -2, -3
    print_permutation( &input, &directions );
    res.push( input.clone() );

    loop {
        let largest = find_largest_mobile_element(&input, &directions);
        let direction = largest.direction;
        let mobile_position = largest.position;

        if direction == NotMobile {
            break;
        }

        let swap_result = direction.swap( &mut input, &mut directions, mobile_position );

        if let Err( SwapError(position)) = swap_result {
            return Err(format!("swap permutation error at position {}", position));
        }
        res.push( input.clone() );


        let reset_result = direction.reset( &input, &mut directions, mobile_position );

        if let Err(SwapError(position)) = reset_result {
            return Err(format!("swap permutation error at position {}", position));
        }
        if let Err(ResetError(position)) = reset_result {
            return Err(format!( "reset permutation error at position {}", position));
        }
        //print_permutation( &input, &directions );
    }

    Ok(res)
}




