/*
	Name: test_dangling_pointer
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Demonstrating a dangling pointer
*/

// Main test driver
fn main(){
	// Create a reference of string type
	let dangling_pointer: &String;

    {
        let this_string = String::from("This string");
        dangling_pointer = &this_string;
    } // Since 'this_string' goes out of scope then deallocated automatically

    // Problem: 'dangling_pointer' points to a deallocated memory!
	// Error: 'Borrowed value does not live long enough' 
    println!("{}", dangling_pointer);

	// Solution: see test_arc.rs
}