/*
	Name: test_box_more
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Handling more 'Box' implementations
*/

// Declare a 'Person' struct
struct Person{
    this_name: String,
    this_age: i32,
}

// Declare a 'Cat' struct
struct Cat{
    this_name: String,
    this_age: i32,
}

// Implement some trait objects 
trait AccessDetails{
    fn get_name(&self) -> String;
    fn get_age(&self) -> i32;
}

// Implement trait for 'Cat'
impl AccessDetails for Cat{

    // Accessors 
    fn get_name(&self) -> String{
        return self.this_name.to_owned();
    }

    fn get_age(&self) -> i32{
        return self.this_age;
    }
}

// Implement trait for 'Person'
impl AccessDetails for Person{

    // Accessors 
    fn get_name(&self) -> String{
        return self.this_name.to_owned();
    }

    fn get_age(&self) -> i32{
        return self.this_age;
    }
}

// Get accessor type 
// Size needs to be known on compile time so use Box "smart pointer"
fn get_access_type(this_access: &str) -> Box<dyn AccessDetails>{
    match this_access{
        "Person" => Box::new(Person {
            this_name: "John".to_string(),
            this_age: 30,
        }),
        "Cat" => Box::new(Cat {
            this_name: "Billy".to_string(),
            this_age: 2,
        }),
        _=> panic!(),
    }
}

// Main test driver 
fn main(){
    //let this_person: Person = Person{this_name : "John".to_string(), this_age : 23};

    // Creating a new trait object and returning it
    println!("this persons name is: {}",get_access_type("Person").get_name());
    println!("this cats name is: {}",get_access_type("Cat").get_name());
}