/*
	Name: test_array
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing arrays
*/

// Linear search 
fn linear_search(this_arr: &[u32], to_find: &u32)->i32{
    println!("{}",this_arr[0]);
    let mut found = false;
    let mut pos = 0;
    for element in this_arr{
        if element == to_find{
            return pos;
        }else{
            pos = pos+1;
        }
        pos = pos + 1;
    }
    return -1;
}

// Main test driver
fn main(){
    let this_arr_size = 4;
    let this_arr = [4,6,8,3];

    // Using for loop 
    println!("Now using for loop");
    for element in this_arr{
        println!("{}",element);
    }

    // Using iteration
    println!("Now using iteration");
    for element in this_arr.iter(){
        println!("{}",element)
    }

    // Using closures 
    println!("Now using clousers");
    let print_number = move ||for element in this_arr.iter(){println!("{}",element)};
    print_number();

    // Using while loop 
    println!("Now using while loop");
    let mut this_index = 0; 
    while this_index < this_arr.len(){
        println!("{}",this_arr[this_index]);
        this_index = this_index + 1;
    }

    // Testing pointer dereference
    unsafe{
        let mut this_arr_ptr = this_arr.as_ptr();
        println!("The dereferenced pointer at index 0 is: {:?}",*this_arr_ptr);
        println!("The dereferenced pointer at index 1 is: {:}",*this_arr_ptr.offset(1));
    }

    // Testing linear search
    println!("Testing linear search");
    let found = linear_search(&this_arr,&4);
    println!("Found the number 4 at position {} in this array : {:?} ",linear_search(&this_arr,&4),&this_arr);
    println!("The size of this array is: {}",this_arr.len());
    
    let new_arr = this_arr.clone();
    println!("The copied array is: {:?}",new_arr);
}