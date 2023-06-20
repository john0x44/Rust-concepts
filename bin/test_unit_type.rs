/*
	Name: test_unit_type
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing unit type
*/

// Returning a unit type!
fn report_string() -> (){
	println!("Hello world!");
}

// Main test driver
fn main(){
	println!("This is a unit type sometimes they are useful! {:?}", report_string());
}