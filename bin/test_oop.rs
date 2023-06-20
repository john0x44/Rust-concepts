/*
	Name: test_oop
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing OOP paradigm
*/

// Declare 'Person' struct or 'object'
struct Person{
    age: i32,
    name: String,
}

// Implement 
impl Person{

    // Accessors 
    fn get_name(&self) -> String {
        return self.name.to_string();
    }

    fn get_age(&self) -> i32{
        return self.age;
    }

    // Mutators 
    fn set_name(&mut self, this_name: String){
        self.name = this_name;
    }

    fn set_age(&mut self, this_age: i32){
        self.age = this_age;
    }
}

// Report person name, and age 
fn report_person_details(person: &Person){

    // Test some accessors
    println!("This person is called: {}, who has a age of: {}",person.get_name(),person.get_age());
}

fn main(){
    let mut new_person: Person = Person{age: 32, name: String::from("John")};
    report_person_details(&new_person);

    // Test some mutators 
    new_person.set_name(String::from("Roger"));
    new_person.set_age(39);

    // report mutated 'new_person' object
    report_person_details(&new_person);
}