/*
	Name: test_option
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing Rust 'option'
*/

// Declare a struct 
struct Person{
    name: Option<String>,
    age: i32,
}

// Main test driver
fn main(){
    let new_person: Person = Person{name : None, age: 25};
    println!("The persons age is {}",newPerson.age);
    match newPerson.name{
        Some(name) => println!("The person provided a name which is: {}",name),
        None => println!("The person did not provide a name!"),
    }
}