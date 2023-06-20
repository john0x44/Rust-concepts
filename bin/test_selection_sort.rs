/*
	Name: test_selection_sort
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Implementing and testing selection sort algorithm
*/

// Implement selection sort
fn selection_sort(arr: &mut [i32]){
    let mut start_scan = 0;
    let mut min_index = 0;
    let mut min_value = 0;
    while start_scan < arr.len()-1 {
        min_index = start_scan;
        min_value = arr[start_scan];
        let mut index = start_scan+1;
        while index < arr.len() {
            if arr[index] < min_value{
                min_value = arr[index];
                min_index = index;
            }
            index = index + 1;
        }
        arr[min_index] = arr[start_scan];
        arr[start_scan] = min_value;
        start_scan = start_scan + 1;
    }
}

// Main test driver
fn main(){
    let mut my_arr = [1,2,3,-4,-99,199,-999,0,-33,-9999];
    selection_sort(&mut my_arr);
    println!("{:?}",my_arr);
}