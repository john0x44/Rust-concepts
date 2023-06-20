/*
	Name: test_copy
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Test copy
*/

// Main test driver
fn main(){
    let a: i32 = 3454;
    
    // Implicitly copies using assingment operator
    let b = a;
    println!("{}", b);
}