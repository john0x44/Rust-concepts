/*
	Name: test_ownership
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing Rust 'ownership'
*/

// Report ownership function
fn report_ownership(this_var: i8){
    println!("This 'report_ownership' function has ownership of {}",this_var);
}// Once goes out of scope the variable this_var is deleted automatically or 'dropped'

// Main test driver 
fn main(){

    // 'this_var' is in scope of 'main'
    let this_var = 4;

    let this_string = String::from("This string");
    // 'this_string' "lives" on the heap ; since allocated on the heap will have 'ownership' to 'this_string'
    //let this_other_string = this_string;

    // Problem: wont compile since 'this_string' has its ownership changed
    // println!("{}",this_string);

    // Solution: use clone() instead! 
    // let this_other_string = this_string.clone();
    // println!("{}",this_other_string);

    report_ownership(this_var);

    // new scope block
    {
        let new_var = this_var;
        report_ownership(new_var);
    }

    // will NOT compile due to ownership
    // println!("{}",new_var);
}