/*
	Name: test_clone
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Test copy
*/

// Main test driver
fn main(){
    let a: String = String::from("Hello world!");

    // Explicitly cloning ; copy the pointer to the string in heap
    let b = a.clone(); 

    println!("{}", b);
}