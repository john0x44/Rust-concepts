/*
	Name: test_more_strings
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing and implementing more string methods
*/

// Report string 
fn report_string(string: String){
    println!("{:?}",string);
}

// Main test driver
fn main(){
    // Heap-allocated that is mutable (pointer to the characters in the heap)
    let mut this_string: String = String::from("Hello world!");

    // UTF-8 bytes stored into memory and immutable
    let this_other_string: &str = "Hello world!";

    // String slice
    println!("{:?}", &this_string[0..5]);

    // Testing into
    let s: Box<str> = "Hello world!".into();
    println!("{:?}",&s);

    // Testing 'push_str()'
    this_string.push_str(" Again!");
    println!("{:?}",this_string);

    // Testing 'replace()'
    let new_string = this_string.replace(" Again!","");
    println!("{:?}",new_string);

    // Testing 'to_owned'
    let other_string: &str = "Hello world!";
    report_string(other_string.to_owned());
    report_string(String::from(other_string));
    for chr in other_string.chars(){
        println!("{:?}",chr);
    }
}