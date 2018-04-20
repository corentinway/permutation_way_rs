

use mobility::Mobility;
use mobility::Mobility::*;

/// Largest element found for permutation
pub struct Largest {
    pub direction : Mobility,
    pub position : usize
}

pub fn print_permutation<T>( input : &Vec<T>, directions : &Vec<Mobility>)
    where T : Sized
{
    assert_eq!(input.len(), directions.len());
    let mut str = String::new();

    for index in 0..input.len() {

        let data : T = *input.get( index ).unwrap();
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

pub fn find_largest_mobile_element<T>(input: &Vec<T>, directions : &Vec<Mobility> ) -> Largest 
    where T : PartialOrd
{

    if input.len() == 0 {
        return Largest {
            direction: NotMobile,
            position: 0
        };
    }

    let mut mobile_position : usize = 0;
    let mut max_value = input.get(mobile_position ).unwrap();
    let mut max_direction : Mobility = directions.get(mobile_position ).unwrap().clone();

    for index in 1..input.len() {
        let is_mobile = directions.get(index).unwrap() != &NotMobile;
        let current_value = input.get(index).unwrap();

        /*println!("----------------------------------------");
        println!("   is mobile   {}", is_mobile);
        println!("   current val {}", current_value);
        println!("   max         {}", max_value);
        println!("   index       {}", index);
        println!("   mobility    {:?}", directions.get(index) );
        */

        if max_direction == NotMobile || ( is_mobile && current_value > max_value ) {
            max_value = current_value;
            mobile_position = index;
            max_direction = directions.get(index).unwrap().clone();
            //println!( "\tmax found val={}, pos={}, dir={:?}", current_value, mobile_position, max_direction);
        }
    }

    let max_direction = max_direction.clone();

    Largest {
        direction: max_direction,
        position: mobile_position
    }
}

#[cfg(test)]
mod tests {

    use mobility::Mobility;
    use mobility::Mobility::*;
    use super::Largest;
    use super::find_largest_mobile_element as find;

    #[test]
    fn should_find_no_mobile_given_an_empty_input_array() {
        // input
        let input = vec![];
        let directions= vec![];
        // call
        let actual_largest = find(&input, &directions);
        // asserts
        assert_eq!( NotMobile, actual_largest.direction );
        assert_eq!( 0, actual_largest.position );
    }

    #[test]
    #[warn(non_snake_case)]
    fn should_return_NotMobile_given_all_directions_not_mobile() {
        // input
        let input = vec![1, 2, 3];
        let directions= vec![NotMobile, NotMobile, NotMobile];
        // call
        let actual_largest = find(&input, &directions);
        // asserts
        assert_eq!( NotMobile, actual_largest.direction );
        assert_eq!( 2, actual_largest.position );
    }

    #[test]
    #[warn(non_snake_case)]
    fn should_return_the_largest_mobile_eleemnt() {
        // input
        let input = vec![1, 2, 3];
        let directions= vec![NotMobile, Left, Left];
        // call
        let actual_largest = find(&input, &directions);
        // asserts
        // asserts
        assert_eq!( Left, actual_largest.direction );
        assert_eq!( 2, actual_largest.position );
    }
}