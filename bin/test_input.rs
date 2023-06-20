/*
	Name: test_input
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing and implementing input
*/

// Declare a 'Person' struct 
struct Person{
    name: String,
    age: i32,
    number: i32,
}

// Implement some methods
impl Person{
    fn get_age(&self) -> i32{
        return self.age;
    }

    fn get_name(&mut self) -> String{
        return self.name.to_string();
    }

    fn set_name(&mut self,name: String){
        self.name = name;
    }

    fn set_age(&mut self,age: i32){
        self.age = age;
    }

    fn set_number(&mut self,number: i32){
        self.number = number;
    }

    fn get_number(&mut self) -> i32{
        return self.number;
    }
}

// Main test driver
fn main(){
    let mut persons: Vec<Person> = Vec::new();
    let people = 2;
    for i in 0..people{
        let mut age:String = String::new();
        let mut name:String = String::new();
        println!("Please enter person number: {}, age: ",i+1);
        let age_input = std::io::stdin().read_line(&mut age).unwrap();
        let age_conv: i32 = age.trim().parse().unwrap();
        println!("Please enter person number: {} name: ",i+1);
        let name_input = std::io::stdin().read_line(&mut name).unwrap();
        let mut person: Person = Person{name: name.to_string(), age: age_conv, number: i};
        persons.push(person);
    }
    for mut person in persons{
        println!("Person number: {}, age is {}, and name is {}",person.get_number()+1,person.get_age(),person.get_name());
    }
}