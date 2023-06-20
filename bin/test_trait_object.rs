/*
	Name: test_trait_object
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Test trait object
*/

// Declare a trait
trait Person {
    fn person_type(&self) -> String;
}

// Declare some structs
struct Teenager;

impl Person for Teenager {
    fn person_type(&self) -> String {
        "Teenager".to_string()
    }
}

// Main test driver
fn main() {
    let teenager: Teenager = Teenager;
    
    // Referencing trait object
    let person: &dyn Person = &teenager;
    println!("Person type: {}", person.person_type());
}
