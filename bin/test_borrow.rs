/*
	Name: test_borrow
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing Rust 'borrowing'
*/

// Report borrowed or reference '&'
fn report_borrowed(this_borrowed_string: &str){
	println!("{}",this_borrowed_string)
}

// Main test driver
fn main(){
	let this_string = "Hello this is a borrowed string!";
	report_borrowed(&this_string);
} 
