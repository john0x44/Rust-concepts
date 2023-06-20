/*
	Name: test_panic
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing Rust 'panic'
*/

// Main test driver
fn main(){
    {
        let a = "panic!";
        match a {
            "panic!" => panic!("some custom message here!"),
            _ => println!("not panicked! continuing!"),
        }
    }
    
    // Block will panic resulting in an aborted program and will not print out!
    println!("welcome to the main program!");
}