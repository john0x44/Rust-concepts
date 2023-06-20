/*
	Name: test_while_loop
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing and implementing while loop
*/

//Main test driver
fn main(){
    let mut times = 0; 
    while times < 10{
        println!("I said hello world {} times!",times+1);
        times = times + 1;
    }
}