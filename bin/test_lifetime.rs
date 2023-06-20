/*
	Name: test_lifetime
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing and implementing 'lifetime' 
*/

// Add some lifetimes 
// Compiler error before implementing 'lifetime' 
// Error: "this function's return type contains a borrowed value, but the function signature does not say whether it is borrowed from `int_a` or `int_b`"
fn add_together<'a,'b>(int_a: &'a i32, int_b: &'b i32) -> &'a i32{
    return int_a;
}

// Main test driver
fn main() {
    let a: i32 = 1;
    let b: i32 = 2; 

    println!("{:?}",add_together(&a, &b));
} 