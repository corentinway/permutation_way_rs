

mod mobility;
mod utils;

use mobility::Mobility;
use mobility::Mobility::*;
use mobility::create_directions;

use utils::print_permutation;
use utils::find_largest_mobile_element;

pub fn permute( input : Vec<i32> ) -> Vec<Vec<i32>> {
    let mut res = Vec::new();

    let mut input = input;
    input.sort();

    let mut directions = create_directions( &input );

    // push 1, -2, -3
    print_permutation( &input, &directions );
    res.push( input.clone() );

    // ////////////////////////////////////////////////////////

    let largest = find_largest_mobile_element(&input, &directions);

    println!("\tinput      {:?}", input);
    println!("\tdirections {:?}", directions);
    println!("\tlargest    {:?} {}", largest.0, largest.1);
    let direction = largest.0;
    let mobile_position = largest.1;

    assert_eq!(Left, direction);
    assert_eq!(2, mobile_position);
    input.swap(mobile_position - 1, mobile_position);
    directions.swap(mobile_position - 1, mobile_position);
    res.push(input.clone());

    reset_directions_after_left_swap(&input, &mut directions, mobile_position);
    // 1, -3, -2
    print_permutation(&input, &directions);
    // ////////////////////////////////////////////////////////

    let largest = find_largest_mobile_element(&input, &directions);
    println!( "\tinput      {:?}", input );
    println!( "\tdirections {:?}", directions );
    println!( "\tlargest    {:?} {}", largest.0, largest.1 );
    let direction = largest.0;
    let mobile_position = largest.1;

    assert_eq!( Left, direction );
    assert_eq!( 1, mobile_position);
    input.swap( mobile_position - 1, mobile_position );
    directions.swap( mobile_position - 1, mobile_position );
    res.push( input.clone() );

    reset_directions_after_left_swap( &input, &mut directions, mobile_position );
    // 3, 1, -2
    print_permutation( &input, &directions );

    res
}




fn reset_directions_after_left_swap(input : & Vec<i32>, directions: & mut Vec<Mobility>, mobile_position: usize ) {
    let position_after_swap = mobile_position - 1;
    let is_first_element = position_after_swap == 0;


    if is_first_element ||
        // if next element in the same direction is larger than the choosen one
        input.get( position_after_swap -1 ) > input.get(position_after_swap ) {

        if let Some(element) = directions.get_mut( position_after_swap) {
            *element = NotMobile;
        }

        /*let mut element_direction = directions.get(position_after_swap).unwrap();
        element_direction = &Mobility::NotMobile;*/
    }


}
