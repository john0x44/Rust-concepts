/*
	Name: test_vectors
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing Rust 'vector'
*/

// Report vector size 
fn report_vector_size(this_vector: &[i32]){
    println!("this vector: {:?}, size is: {}",this_vector,this_vector.len());
}

// Main test driver 
fn main(){

    // Immutable vector 
    let some_numbers: Vec<i32> = vec![1,2,3,4,5,6,7,8];
    report_vector_size(&some_numbers);
    
    // Create a mutable vector of type i32
    let mut set_numbers: Vec<i32>  = Vec::new();
    set_numbers.push(99);
    set_numbers.push(100);

    report_vector_size(&set_numbers);

    // Test mutator methods
    set_numbers.pop(); 
    set_numbers[0] = 100;
    report_vector_size(&set_numbers);

    // Iterate through the vector 
    for item in set_numbers{
        println!("{}",item);
    }
}