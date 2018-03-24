#[derive(PartialEq, PartialOrd, Debug)]
enum Mobility {
    #[warn(dead_code)]
    Left,
    #[warn(dead_code)]
    Right,
    #[warn(dead_code)]
    NotMobile
}

use Mobility::*;

fn create_directions( input : & Vec<i32> ) -> Vec<Mobility> {
    // all input elements are mobile leftward
    let mut directions: Vec<Mobility> = input.iter().map( |_| Mobility::Left).collect();
    // the first one is not mobile
    directions[0] = Mobility::NotMobile;

    directions
}

pub fn permute( input : Vec<i32> ) -> Vec<Vec<i32>> {
    let mut res = Vec::new();

    let mut input = input;
    input.sort();

    let mut directions = create_directions( &input );

    // push 1, -2, -3
    println!(">first permutation {:?}", input);
    res.push( input.clone() );

    let largest = find_largest_mobile_element(&input, &directions);
    println!( "input      {:?}", input );
    println!( "directions {:?}", directions );
    println!( "largest    {:?} {}", largest.0, largest.1 );
    let direction = largest.0;
    let mobile_position = largest.1;

    assert_eq!( &Left, direction );
    assert_eq!( 2, mobile_position);
    input.swap( mobile_position - 1, mobile_position );
    directions.swap( mobile_position - 1, mobile_position );



    res
}


fn find_largest_mobile_element<'a>( input: &Vec<i32>, directions : &'a Vec<Mobility> ) -> (&'a Mobility, usize) {

    if input.len() == 0 {
        return (&Mobility::NotMobile, 0);
    }

    let mut mobile_position : usize = 0;
    let mut max_value = input.get(mobile_position ).unwrap();
    let mut max_direction : &Mobility = directions.get(mobile_position ).unwrap().clone();

    for index in 1..input.len() {
        let is_mobile = directions.get(index).unwrap() != &Mobility::NotMobile;
        let current_value = input.get(index).unwrap();

        println!("----------------------------------------");
        println!("   is mobile   {}", is_mobile);
        println!("   current val {}", current_value);
        println!("   max         {}", max_value);
        println!("   index       {}", index);

        if is_mobile && current_value > max_value {
            max_value = current_value;
            mobile_position = index;
            max_direction = directions.get(index).unwrap().clone();
            println!( "max found val={}, pos={}, dir={:?}", current_value, mobile_position, max_direction);
        }
    }

    let max_direction = max_direction;

    (max_direction, mobile_position )
}