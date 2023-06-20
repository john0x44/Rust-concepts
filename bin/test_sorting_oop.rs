/*
	Name: test_sorting_oop
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Creating and implementing a SortObject 
*/

// Decalare a struct and add lifetime 
struct SortingObject<'arr>{
    this_arr: &'arr mut [i32],
}

// Implement some methods
impl <'arr>SortingObject<'arr>{

    // Mutators 

    // Bubble sort the array in ascending order
    fn sort_arr(&mut self){
        let arr = &mut self.this_arr;
        let size = arr.len() - 1;
        let __index = 0;
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

    // Find number 
    fn find_number(&mut self, this_value: i32) -> (bool, usize){
        // Sort the array first 
        self.sort_arr();
        match self.bin_search(this_value){
            Some(_value) => return (true, _value),
            None => return (false, 0),
        }
    }
    
    // Binary search in the array 
    fn bin_search(&self,to_find: i32)->Option<usize>{
        let mut first = 0;
        let this_arr = &self.this_arr;
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
}

fn main(){
    let mut array = [1,2,35,-3,0,-99,50,2345,-999];
    let mut this_sorting: SortingObject = SortingObject{this_arr: &mut array};
    println!("Finding number 2: found result: {}, at position: {}",this_sorting.find_number(2).0,this_sorting.find_number(2).1);
    println!("the array after internal mutation is: {:?}",this_sorting.this_arr);
}