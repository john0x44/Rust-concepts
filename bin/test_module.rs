/*
	Name: test_module
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Test nodule
*/

// Declare and initialize a module
mod test_module{

	// Available to application programmer
	// public accessor type is not private anymore!
	pub fn report_message(){
		println!("Welcome application programmer!");
	}

	// Available to the class programmer 
	fn do_something(){
		println!("Welcome class programmer!");
	}
 
	pub fn report_another_message(){
		do_something();
	}
}

// Main test driver
fn main(){
	test_module::report_message();
	test_module::report_another_message();
}