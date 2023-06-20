/*
	Name: test_arc
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Demonstrating 'arc'
*/

// Allow the use of shared ownership
use std::sync::Arc;

// Main test driver
fn main(){

    // Solution: using Arc 'Atomically Reference Counted’
	// Create a reference of string type
	let dangling_pointer: Arc<String>;

    {
        let this_string = Arc::new(String::from("This string"));

        // Clone points to the same location in heap as 'this_string'
        // Less overhead by using clone
        dangling_pointer = Arc::clone(&this_string); 
    } // Since goes out of scope then deallocated automatically

    // Problem: dangling_pointer points to a deallocated memory!
    println!("{}", dangling_pointer);

}