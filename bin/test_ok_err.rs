/*
	Name: test_ok_err
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Implementing and testing 'Ok and Err'
*/

// Divide two integers
fn divide(a: i8, b: i8) -> Result<i8, String> {
    if b == 0 || a == 0{
        Err("Division by zero".to_string()) 
    } else {
        Ok(a / b) 
    }
}

// Main test driver
fn main(){
    let result_div = divide(4, 0);
    match result_div{
        Ok(answer) => println!("Division result is: {}", answer),
        Err(error) => println!("The Error is: {}", error),
	}
}