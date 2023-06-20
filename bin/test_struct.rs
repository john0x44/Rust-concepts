/*
	Name: test_struct
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Test struct
*/

// Declare a 'Person' struct
struct Person{
    name: String,
    age: i32,
}

// Main test driver
fn main(){
    let person:Person = Person{name: "Alice".to_string(), age: 25};
	println!("{} is {} years old!", person.name, person.age);
}