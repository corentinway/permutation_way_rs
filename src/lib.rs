

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
    direction.swap_input( &mut input, mobile_position );
    direction.swap_directions( &mut directions, mobile_position);
    res.push(input.clone());

    direction.reset(&input, &mut directions, mobile_position);
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
    direction.swap_input( &mut input, mobile_position );
    direction.swap_directions( &mut directions, mobile_position);
    res.push( input.clone() );

    direction.reset( &input, &mut directions, mobile_position );
    // 3, 1, -2
    print_permutation( &input, &directions );


    // ////////////////////////////////////////////////////////

    let largest = find_largest_mobile_element(&input, &directions);
    println!( "\tinput      {:?}", input );
    println!( "\tdirections {:?}", directions );
    println!( "\tlargest    {:?} {}", largest.0, largest.1 );
    let direction = largest.0;
    let mobile_position = largest.1;

    assert_eq!( Left, direction );
    assert_eq!( 2, mobile_position);
    direction.swap_input( &mut input, mobile_position );
    direction.swap_directions( &mut directions, mobile_position);
    res.push( input.clone() );

    direction.reset( &input, &mut directions, mobile_position );
    // 3, 1, -2
    print_permutation( &input, &directions );

    // ////////////////////////////////////////////////////////

    let largest = find_largest_mobile_element(&input, &directions);
    println!( "\tinput      {:?}", input );
    println!( "\tdirections {:?}", directions );
    println!( "\tlargest    {:?} {}", largest.0, largest.1 );
    let direction = largest.0;
    let mobile_position = largest.1;

    assert_eq!( Right, direction );
    assert_eq!( 0, mobile_position);
    direction.swap_input( &mut input, mobile_position );
    direction.swap_directions( &mut directions, mobile_position);
    res.push( input.clone() );

    direction.reset( &input, &mut directions, mobile_position );
    // 3, 1, -2
    print_permutation( &input, &directions );

    // ////////////////////////////////////////////////////////

    let largest = find_largest_mobile_element(&input, &directions);
    println!( "\tinput      {:?}", input );
    println!( "\tdirections {:?}", directions );
    println!( "\tlargest    {:?} {}", largest.0, largest.1 );
    let direction = largest.0;
    let mobile_position = largest.1;

    assert_eq!( Right, direction );
    assert_eq!( 1, mobile_position);
    direction.swap_input( &mut input, mobile_position );
    direction.swap_directions( &mut directions, mobile_position);
    res.push( input.clone() );

    direction.reset( &input, &mut directions, mobile_position );
    // 3, 1, -2
    print_permutation( &input, &directions );


    let largest = find_largest_mobile_element(&input, &directions);
    println!( "\tinput      {:?}", input );
    println!( "\tdirections {:?}", directions );
    println!( "\tlargest    {:?} {}", largest.0, largest.1 );
    let direction = largest.0;
    let mobile_position = largest.1;

    assert_eq!( NotMobile, direction );
    assert_eq!( 0, 2);




    res
}




