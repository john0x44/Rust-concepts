/*
	Name: test_pointer
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing pointer
*/

// Main test driver
fn main() {
    let number = 12345678;
    let num_ptr = &number as *const i32;
    println!("Pointer value : {:?}, dereferenced pointer {:?}", num_ptr, unsafe{*num_ptr}); 
}