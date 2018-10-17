///
/// This enum provide indication whether an element in the input
/// vector should swap with the next element at its left of at its
/// right. More over, the next element can be set as not mobile as well.
///
#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub enum Mobility {
    Left,
    Right,
    NotMobile,
}

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub enum MobilityError {
    /// error if there is a try to swap. The error contains the position of the element to swap
    SwapError(usize),
    ResetError(usize),
}

use Mobility::*;

impl Mobility {
    pub fn swap<T>(
        &self,
        input: &mut Vec<T>,
        directions: &mut Vec<Mobility>,
        mobile_position: usize,
    ) -> Result<(), MobilityError> {
        match *self {
            Left => {
                if mobile_position == 0 {
                    return Err(MobilityError::SwapError(0));
                }
                input.swap(mobile_position - 1, mobile_position);
                directions.swap(mobile_position - 1, mobile_position)
            }
            Right => {
                if mobile_position == input.len() - 1 {
                    return Err(MobilityError::SwapError(mobile_position));
                }
                input.swap(mobile_position, mobile_position + 1);
                directions.swap(mobile_position, mobile_position + 1)
            }
            NotMobile => (),
        }

        Ok(())
    }

    pub fn reset<T>(
        &self,
        input: &Vec<T>,
        directions: &mut Vec<Mobility>,
        mobile_position: usize,
    ) -> Result<(), MobilityError>
    where
        T: PartialOrd,
    {
        let position_after_swap = self.get_position_after_swap(mobile_position)?;

        let is_first_element = position_after_swap == 0;
        let is_last_element = position_after_swap == input.len() - 1;

        if is_first_element || is_last_element ||
            // if next element in the same direction is larger than the choosen one
            self.get_next_element( &input, position_after_swap)? > input.get(position_after_swap )
        {
            if let Some(element) = directions.get_mut(position_after_swap) {
                *element = NotMobile;
            }
        }

        // After each step, all elements greater than the chosen element have their directions
        // set to positive or negative,
        // according to whether they are between the chosen element and the start
        // or the end of the permutation respectively.

        for index in 0..position_after_swap {
            if input.get(index) > input.get(position_after_swap) {
                if let Some(element) = directions.get_mut(index) {
                    *element = Right;
                }
            }
        }
        for index in position_after_swap + 1..input.len() {
            if input.get(index) > input.get(position_after_swap) {
                if let Some(element) = directions.get_mut(index) {
                    *element = Left;
                }
            }
        }

        Ok(())
    }

    fn get_next_element<'a, T>(
        &self,
        input: &'a Vec<T>,
        position_after_swap: usize,
    ) -> Result<Option<&'a T>, MobilityError>
    where
        T: PartialOrd,
    {
        match *self {
            Left => Ok(input.get(position_after_swap - 1)),
            Right => Ok(input.get(position_after_swap + 1)),
            NotMobile => Err(MobilityError::ResetError(position_after_swap)),
        }
    }

    fn get_position_after_swap(&self, mobile_position: usize) -> Result<usize, MobilityError> {
        match *self {
            Left => Ok(mobile_position - 1),
            Right => Ok(mobile_position + 1),
            NotMobile => Err(MobilityError::SwapError(mobile_position)),
        }
    }
}

pub fn create_directions<T>(input: &Vec<T>) -> Vec<Mobility> {
    if input.len() == 0 {
        return vec![];
    }

    // all input elements are mobile leftward
    let mut directions: Vec<Mobility> = input.iter().map(|_| Left).collect();
    // the first one is not mobile
    directions[0] = NotMobile;

    directions
}

#[cfg(test)]
mod tests {
    use super::Mobility::*;
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

    #[test]
    fn should_swap_mobility_to_left() {
        // input
        let mut input = vec![1, 2, 3];
        let mut directions = vec![NotMobile, Left, Right];
        let mobile_position = 1;
        // call
        Left.swap(&mut input, &mut directions, mobile_position);
        // assertions
        assert_eq!(vec![2, 1, 3], input);
        assert_eq!(vec![Left, NotMobile, Right], directions);
    }

    #[test]
    fn should_swap_mobility_to_right() {
        // input
        let mut input = vec![1, 2, 3];
        let mut directions = vec![Right, NotMobile, NotMobile];
        let mobile_position = 0;
        // call
        Right.swap(&mut input, &mut directions, mobile_position);
        // assertions
        assert_eq!(vec![2, 1, 3], input);
        assert_eq!(vec![NotMobile, Right, NotMobile], directions);
    }

    #[test]
    fn should_not_swap_leftward_the_first_element() {
        // input
        let mut input = vec![1, 2, 3];
        let mut directions = vec![NotMobile, Left, Right];
        let mobile_position = 0;
        // call
        let actual = Left.swap(&mut input, &mut directions, mobile_position);
        // assertions
        assert_eq!(Err(MobilityError::SwapError(0)), actual);
    }
    #[test]
    fn should_not_swap_rightward_the_last_element() {
        // input
        let mut input = vec![1, 2, 3];
        let mut directions = vec![NotMobile, Left, Right];
        let mobile_position = 2;
        // call
        let actual = Right.swap(&mut input, &mut directions, mobile_position);
        // assertions
        assert_eq!(Err(MobilityError::SwapError(2)), actual);
    }
}
