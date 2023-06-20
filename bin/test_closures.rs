/*
	Name: test_closures
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Test closures
*/

// Report closure 
fn process_string<T>(first_string: &str, this_func: T) -> String
where T: Fn(&str)-> String
{
    let this_new_string = this_func(first_string);
    return this_new_string;
}

// Main test driver
fn main(){
    let age: i8 = 21;
    let this_arr = [1,2,3,4,5,6,7,11,12];

    // Test a closure or anonymous function on 'age'
    println!("is_21 = {}", (|| -> bool {return age <= 21;})());

    // Print the sum of 'this_arr' in a closure 
    println!("the sum of 'this_arr' is: {:?}",(|| -> i32 {
        let mut total: i32 = 0; 
        for number in this_arr{
            total = total + number;
        }return total;})()
    );

    // Passing a closure to a function
    let input_string = "Hello";
    let closure_func = |input_string: &str| -> String {return format!("{} World",input_string);};
    println!("Result: {}", process_string(input_string, closure_func));
}