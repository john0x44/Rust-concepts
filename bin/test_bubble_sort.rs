/*
	Name: test_bubble_sort
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Implementing bubble sort algorithm
*/

// Bubble sort 
fn bub_sort(arr: &mut [i32]) {
    let size = arr.len() - 1;
    let _index = 0;
    loop {
        let mut swap = false;
        for index in 0..size {
            if arr[index] > arr[index + 1] {
                arr.swap(index, index + 1);
                swap = true;
            }
        }
        if !swap {break;}
    }
}

// Main test driver
fn main(){
    let mut this_arr = [9,-3,0,1,5,-3,-939,100,23,2,-1000,-6,-99,-7];
    bub_sort(&mut this_arr);
    println!("newly sorted array is: {:?}",this_arr);
}