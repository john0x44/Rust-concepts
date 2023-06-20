/*
	Name: test_tuple
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing tuples
*/

// Return a tuple
fn get_tuple() -> (i32,i32,i32){
	return (3,2,1);
}

// Main test driver
fn main(){
	let values = (1,2,3,4,6);
	println!("{:?}",values.0);
	println!("{:?}",get_tuple().0);
}