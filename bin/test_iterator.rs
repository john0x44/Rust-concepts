/*
	Name: test_iterator
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing and implementing iterators
*/

use std::collections::HashMap; 

// Main test driver
fn main(){
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("Person1",24);
    map.insert("Person2",23);
    map.insert("Person3",27);
    
    // Iterate through the hashmap 
    // Remember: 'iter()' borrows! cannot change
    for person in map.iter(){
        println!("{}, has a value of {}",person.0,person.1);
    }

    // Testing 'next()'
    let mut iterator = map.iter();
    println!("Next value in iterator is: {:?}",iterator.next());

    // Remember: 'into_iter()' takes ownership then dispatches!
    for person in map.into_iter(){
        println!("{}, has a value of {}",person.0,person.1);
    } // Dispatches after scope 
    // Complie error!
    //println!("{:?}",map.get("Person1"));
}