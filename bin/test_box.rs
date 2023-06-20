/*
	Name: test_box
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing Rust 'Box'
*/

// Main test driver
fn main(){
    // Box or reference allows dynamic dispatch
    // Allocates into heap and stores 'this_item' into it
    let this_item = Box::new(String::from("hello from heap!"));
    let this_ptr = Box::into_raw(this_item.clone());

    // Now 'this_item' Box contains a smart pointer to the heap
    println!("{}, address pointer: {:p}",this_item,this_ptr);
}