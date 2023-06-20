/*
	Name: test_asccociated_type
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing asscociated types 
*/

// Declare a struct
struct Person{
    age: i32;
}

// Declare a trait for 'Person'
trait Details{

    // Add an associated type
    type AgeType; 

    // Acessor function signature
    fn get_age(&self) -> AgeType;
}

// Implement the 'Details' trait 
impl Details for Person{

    // Define the associated type
    type AgeType = i32;

    // Define the accessor 
    fn get_age(&self) -> AgeType {
        return self.age
    }
}
// Main test driver
fn main(){
    let person: Person = Person{age: 28};
    println!("{}",person.get_age());
}