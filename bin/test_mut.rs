/*
	Name: test_mut
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing 'mut' modifier 
*/

// Main test driver
fn main(){
    // Will not compile! due to default immutable type

    // let this_str = "Hello World!";
    // this_str = "Goodbye world!"; 

    // Mut modifier added 
    let mut this_str = "Hello World!";
    this_str = "Goodbye World!";
    println!("{}",this_str);
}