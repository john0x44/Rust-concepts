/*
	Name: test_string
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing some string methods
*/

// Report size of string
fn report_string_size(this_string: &str){
    println!("The string: {}, has a size of: {}",this_string,this_string.len());
}

// Report each character in a string
fn report_string_chars(this_string: &str){
    let mut size = 0;
    let vec_chars: Vec<char> = this_string.chars().collect();
    while size < this_string.len() {
        println!("{}",vec_chars[size]);
        size = size + 1;
    }
}

// Report a string find using match
fn report_string_find(this_string: &str, to_find: &str){
    println!("Trying to find the word '{}', on this string: '{}'",to_find,this_string);
    match this_string.find(to_find){
        Some(pos) => println!("Found word: {}, at position: {}",to_find,pos),
        None => println!("Could not find the word: {}",to_find),
    }
}

// Report a character at a specific index
fn report_char_at_index(this_string: &str, this_index: usize){
    if this_index < this_string.len(){
        println!("Found character {:?} at position {}",this_string.chars().nth(this_index),this_index);
    }else{
        println!("Index is not valid!");
    }
}

// Main test driver 
fn main(){
    
    // Make a string literal ; allocated in heap
    let test_string: &str = "This is a long string!";

    // Make a mutable String object from a literal
    let mut test_string1: String = String::from(test_string);

    // Make a mutable String object from a literal 
    let mut test_string2: String = test_string.to_string();

    // Run some report tests 
    report_string_size(&test_string);
    test_string1.push_str(" this is more!");
    report_string_size(&test_string1);
    report_string_size(&test_string2);
    report_string_chars(&test_string2);
    report_string_find(&test_string1,"more!");
    report_char_at_index(&test_string1,2);

    // Test clear method and string references
    test_string1.clear();
    test_string2.clear();
    report_string_find(&test_string1,"more!");
}