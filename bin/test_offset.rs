/*
	Name: test_offset
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Test offset
*/

// Main test driver
fn main(){
    let this_string: &str = "Hello world!";
    (||{
        let mut this_offset = 0;
        for _n in this_string.chars(){
            println!("{:?}",unsafe{(*this_string.as_ptr().offset(this_offset)) as char});
            this_offset = this_offset + 1;
        }
    })();
}