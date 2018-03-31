#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub enum Mobility {
    Left,
    Right,
    NotMobile
}


impl Mobility {

    pub fn swap_input( &self, input : &mut Vec<i32 >, mobile_position : usize) {
        match *self {
            Left => input.swap( mobile_position - 1, mobile_position ),
            Right => input.swap( mobile_position, mobile_position + 1 ),
            NotMobile => (),
        }
    }
    pub fn swap_directions( &self, directions : &mut Vec<Mobility>, mobile_position : usize) {
        match *self {
            Left => directions.swap( mobile_position - 1, mobile_position ),
            Right => directions.swap( mobile_position, mobile_position + 1 ),
            NotMobile => (),
        }
    }

    pub fn reset( &self, input : &Vec<i32>, directions : &mut Vec<Mobility>, mobile_position : usize) {

        let position_after_swap = self.get_position_after_swap(mobile_position);

        let is_first_element = position_after_swap == 0;
        let is_last_element = position_after_swap == input.len() - 1;


        if is_first_element || is_last_element ||
            // if next element in the same direction is larger than the choosen one
            self.get_next_element( &input, position_after_swap) > input.get(position_after_swap ) {

            if let Some(element) = directions.get_mut( position_after_swap) {
                *element = NotMobile;
            }

        }

        // After each step, all elements greater than the chosen element have their directions
        // set to positive or negative,
        // according to whether they are between the chosen element and the start
        // or the end of the permutation respectively.

        for index in 0..position_after_swap {
            if input.get(index) > input.get(position_after_swap ) {
                if let Some(element) = directions.get_mut( index) {
                    *element = Right;
                }
            }
        }
        for index in position_after_swap+1..input.len() {
            if input.get(index) > input.get(position_after_swap ) {
                         if let Some(element) = directions.get_mut( index) {
                    *element = Left;
                }
            }
        }

    }

    fn get_next_element<'a>( &self, input: &'a Vec<i32>, position_after_swap : usize ) -> Option<&'a i32> {
        match *self {
            Left => input.get(position_after_swap - 1),
            Right => input.get( position_after_swap + 1),
            NotMobile => panic!("should never happen"),
        }
    }

    fn get_position_after_swap( &self, mobile_position :usize ) -> usize {
        match *self {
            Left => mobile_position - 1,
            Right => mobile_position + 1,
            NotMobile => panic!("the element should not had been swapped"),
        }
    }
}


use Mobility::*;

pub fn create_directions( input : & Vec<i32> ) -> Vec<Mobility> {
    // all input elements are mobile leftward
    let mut directions: Vec<Mobility> = input.iter().map( |_| Left).collect();
    // the first one is not mobile
    directions[0] = NotMobile;

    directions
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_initial_direction_vec_with_the_same_len_og_the_given_input_vec() {
        // input
        let input = vec![1, 2, 3];
        // call
        let directions = create_directions(&input);
        // assertions
        assert_eq!(3, directions.len());
        assert_eq!(Some(&NotMobile), directions.get(0));
        assert_eq!(Some(&Left), directions.get(1));
        assert_eq!(Some(&Left), directions.get(2));
    }
}