/*
	Name: test_hashmap
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing and implementing hashmaps
*/

use std::collections::HashMap; 

// Report the largest balance 
fn report_larger_bal(hashmap: &HashMap<&str, f32>){
    let mut largest: f32 = 0.0;
    let mut person_name: &str = "None";
    for person in hashmap{
        if largest < *person.1{
            largest = *person.1;
            person_name = person.0;
        }
    }
    match person_name{
        "None" => println!("Nobody has the highest balance!"),
        _ => println!("{:?}, has the highest balance with a balance of {}",person_name,largest),
    }
}

// Main test driver
fn main(){
    let mut balances: HashMap<&str, f32> = HashMap::new();
    balances.insert("John", 100.01);
    balances.insert("Alice", 100.00);
    balances.insert("Bob", 99.99);
    println!("{:?}",balances.get("John").unwrap());
    let person = "Smith";
    match balances.get(person){
        Some(bal) => println!("{} has a balance of {}",person,bal),
        None => println!("There is no balance for {}",person),
    }
    report_larger_bal(&balances);
}