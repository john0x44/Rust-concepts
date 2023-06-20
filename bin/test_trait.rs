/*
	Name: test_trait
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing traits
*/

// Declare 'Person' struct
struct Person{
    age: i32,
}

// Declare a trait 
trait Details {
    fn get_age(&self) -> i32;
}

// Implement 'Details' trait for 'this' Person
impl Details for Person{
    
    // Accessors 
    fn get_age(&self) -> i32 {
        return self.age;
    }
}

// Main test driver 
fn main(){
    let person: Person = Person{age: 32};
    
    // Test trait acessor 
    println!("this persons age is: {}",person.get_age());
}