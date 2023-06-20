/*
	Name: test_scope
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Test scope
*/

// Main test driver
fn main(){
    let this_value = 42;
    {
        // 'this_value' is in scope
        let this_answer = 42;
        println!("The answer to life is: {:?}",this_value);
    }

    // 'this_answer' is now out of scope!
    // Will error!
    // println!("The answer to life is: {:?}",this_answer);
    println!("The answer to life is: {:?}", this_value);
}