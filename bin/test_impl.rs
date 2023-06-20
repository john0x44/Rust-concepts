/*
	Name: test_impl
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing 'impl'
*/

// Declare struct
struct Person{
    age: i32,
}

// Utilizing 'impl' for 'Person' struct
impl Person{

    // mutator 
    fn assign_age(&mut self, this_age: i32){
        self.age = this_age;
    }

    // accessor
    fn get_age(&self) -> i32{
        return self.age;
    }
}

// Main test driver 
fn main(){
    let mut person: Person = Person{age: 32};

    // Test mutator
    person.assign_age(21);
    
    // Test acessor 
    println!("this persons age is: {}",person.get_age());
}