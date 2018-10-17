extern crate permutation_way;

use permutation_way::PermutationIterator;

use std::cmp::Ordering;

#[derive(Eq, Clone, Debug)]
struct Person {
    id: u32,
    name: String,
    height: u32,
}

impl Ord for Person {
    fn cmp(&self, other: &Person) -> Ordering {
        self.height.cmp(&other.height)
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Person) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Person) -> bool {
        self.height == other.height
    }
}

#[test]
fn should_find_6_permutation_given_a_vec_of_3_float_elements_with_iterator() {
    // input
    let input = vec![
        Person {
            id: 1,
            name: String::from("john"),
            height: 160,
        },
        Person {
            id: 1,
            name: String::from("marie"),
            height: 166,
        },
        Person {
            id: 1,
            name: String::from("paul"),
            height: 188,
        },
    ];
    // call
    let mut iterator = PermutationIterator::new(input);
    // assertions
    assert_eq!(
        Some(vec![
            Person {
                id: 1,
                name: String::from("john"),
                height: 160,
            },
            Person {
                id: 1,
                name: String::from("marie"),
                height: 166,
            },
            Person {
                id: 1,
                name: String::from("paul"),
                height: 188,
            },
        ]),
        iterator.next()
    );
    assert_eq!(
        Some(vec![
            Person {
                id: 1,
                name: String::from("john"),
                height: 160,
            },
            Person {
                id: 1,
                name: String::from("paul"),
                height: 188,
            },
            Person {
                id: 1,
                name: String::from("marie"),
                height: 166,
            },
        ]),
        iterator.next()
    );
    assert_eq!(
        Some(vec![
            Person {
                id: 1,
                name: String::from("paul"),
                height: 188,
            },
            Person {
                id: 1,
                name: String::from("john"),
                height: 160,
            },
            Person {
                id: 1,
                name: String::from("marie"),
                height: 166,
            },
        ]),
        iterator.next()
    );
    assert_eq!(
        Some(vec![
            Person {
                id: 1,
                name: String::from("paul"),
                height: 188,
            },
            Person {
                id: 1,
                name: String::from("marie"),
                height: 166,
            },
            Person {
                id: 1,
                name: String::from("john"),
                height: 160,
            },
        ]),
        iterator.next()
    );
    assert_eq!(
        Some(vec![
            Person {
                id: 1,
                name: String::from("marie"),
                height: 166,
            },
            Person {
                id: 1,
                name: String::from("paul"),
                height: 188,
            },
            Person {
                id: 1,
                name: String::from("john"),
                height: 160,
            },
        ]),
        iterator.next()
    );
    assert_eq!(
        Some(vec![
            Person {
                id: 1,
                name: String::from("marie"),
                height: 166,
            },
            Person {
                id: 1,
                name: String::from("john"),
                height: 160,
            },
            Person {
                id: 1,
                name: String::from("paul"),
                height: 188,
            },
        ]),
        iterator.next()
    );
    assert_eq!(None, iterator.next());
    assert_eq!(false, iterator.has_errors());
}
