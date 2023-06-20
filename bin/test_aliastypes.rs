/*
	Name: test_aliastypes
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing an alias type
*/

// Main test driver
fn main(){

    // Make some type alias
    type STRING = String;
    type I32 = i32; 
    let this_string: STRING = String::from("Hello world!");
    let this_int: I32 = I32::MAX;

    println!("{}",this_string);
    println!("{}",this_int);
}