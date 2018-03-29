

use mobility::Mobility;
use mobility::Mobility::*;

pub fn print_permutation( input : &Vec<i32>, directions : &Vec<Mobility>) {
    assert_eq!(input.len(), directions.len());
    let mut str = String::new();

    for index in 0..input.len() {

        let data : i32 = *input.get( index ).unwrap();
        let data = data.to_string();

        match directions.get(index).unwrap() {
            &NotMobile => str.push_str( &data),
            &Left => {
                str.push_str( "-");
                str.push_str( &data)
            },
            &Right => {
                str.push_str( "+");
                str.push_str( &data)
            },
        }

        str.push_str( ", ")
    }

    println!( "permutation {}", str );

}

pub
fn find_largest_mobile_element(input: &Vec<i32>, directions : &Vec<Mobility> ) -> (Mobility, usize) {

    if input.len() == 0 {
        return (NotMobile, 0);
    }

    let mut mobile_position : usize = 0;
    let mut max_value = input.get(mobile_position ).unwrap();
    let mut max_direction : Mobility = directions.get(mobile_position ).unwrap().clone();

    for index in 1..input.len() {
        let is_mobile = directions.get(index).unwrap() != &NotMobile;
        let current_value = input.get(index).unwrap();
        /*
        println!("----------------------------------------");
        println!("   is mobile   {}", is_mobile);
        println!("   current val {}", current_value);
        println!("   max         {}", max_value);
        println!("   index       {}", index);
        */

        if is_mobile && current_value > max_value {
            max_value = current_value;
            mobile_position = index;
            max_direction = directions.get(index).unwrap().clone();
            println!( "\tmax found val={}, pos={}, dir={:?}", current_value, mobile_position, max_direction);
        }
    }

    let max_direction = max_direction.clone();

    (max_direction, mobile_position )
}
