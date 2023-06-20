/*
	Name: test_derived_trait
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Implementing a derived trait
*/

// Derive 'Debug' & 'Clone' for 'Person' struct
#[derive(Debug,Clone)]
struct Person {
    name: String,
    age: u32,
}

// Main test driver
fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 25,
    };

    // Testing derived 'Debug' trait 
    println!("T{:?}", person);

    // Testing derived 'Clone' trait
    let mut another_person = person.clone();
    another_person.name = "Bob".to_string();
    println!("This other person's name is: {}",another_person.name);
}