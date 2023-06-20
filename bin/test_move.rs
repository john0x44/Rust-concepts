/*
	Name: test_move
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing and implementing move 
*/

use std::thread; 
use std::time::Duration;

// Main test driver 
fn main(){
    // Move these to thread enviroment
    let mut balance = 100;
    let this_name = "John";

    // Remember: thread execution is at random
    for i in 0..10{
        thread::spawn(move ||{
            println!("Hello {}, from this thread enviroment number: {}, your balance is {}",this_name,i,balance);
            thread::sleep(Duration::from_millis(10));
        });
    }
}
