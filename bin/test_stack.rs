/*
	Name: test_stack
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Demonstrating Rust stack
*/

// Report a message
fn report_message() {
    println!("Hello world again!"); // Function call pushes a new stack frame
    // When this function completes, its stack frame is popped off the stack
}

// Main test driver
fn main() {
    println!("Hello World!"); // Function call pushes a new stack frame
    report_message(); // Function call pushes a new stack frame
    // When this function call completes, its stack frame is popped off the stack
    // When the main() function completes, its stack frame is popped off the stack
}