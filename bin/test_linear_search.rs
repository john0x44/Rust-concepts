/*
	Name: test_linear_search
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing linear search 
*/

// Linear search 
fn linear_search(this_arr: &[u32], to_find: &u32) -> (bool,i32) {
    let mut found = false;
    let mut pos = 0; 
    for element in this_arr{
        if element == to_find{
            found = true;
            return (found,pos);
        }else{
            pos = pos + 1;
        }
    }
    return (found,-1);
}

// Main test driver 
fn main(){
    println!("Testing linear search");
    let this_arr = [1,2,3,4,5,6,7,8,9,10];
    let to_search = 10;
    let result = linear_search(&this_arr, &to_search);
    match result.0{
        false => println!("Did not find the value: {}",to_search),
        true => println!("Found the value: {} at position: {}",to_search,result.1),
    };
    }