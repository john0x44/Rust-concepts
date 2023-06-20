/*
	Name: test_thread
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Test thread
*/

use std::thread;

// Main test driver
fn main(){
    thread::spawn(|| {println!("Hello from this thread!")});
}