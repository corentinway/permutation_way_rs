#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub enum Mobility {
    #[warn(dead_code)]
    Left,
    #[warn(dead_code)]
    Right,
    #[warn(dead_code)]
    NotMobile
}

use Mobility::*;

pub fn create_directions( input : & Vec<i32> ) -> Vec<Mobility> {
    // all input elements are mobile leftward
    let mut directions: Vec<Mobility> = input.iter().map( |_| Left).collect();
    // the first one is not mobile
    directions[0] = NotMobile;

    directions
}