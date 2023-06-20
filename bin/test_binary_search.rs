/*
	Name: test_binary_search
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing binary search algorithm
*/
use std::time::{Duration, Instant};

// Binary search 
fn bin_search(this_arr: &[usize], to_find: usize) -> Option<usize> {
    let mut first = 0;
    let mut last = this_arr.len() - 1;
    while first <= last {
        let mid = (first + last) / 2;
        if this_arr[mid] == to_find {
            return Some(mid);
        } else if this_arr[mid] > to_find {
            last = mid - 1;
        } else {
            first = mid + 1;
        }
    }
    return None;
}

// Main test driver
fn main(){
    // Remember: array needs to be sorted 
    let this_arr = [1,2,3,4,5,6,7,8,9,10];
    let to_find = 2;
    let found_pos = bin_search(&this_arr,to_find);
    
    let duration = start.elapsed();
    // Matching on return option
    match found_pos {
        Some(_value) => println!("Found {} at index: {}",to_find,_value),
        None=> println!("Not found!"),
    }
}