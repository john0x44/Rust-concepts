/*
	Name: test_concurrency
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Implementing concurrency
*/

use std::thread;
use std::time::Duration; 

// Main test driver
fn main(){
    
    // Problem: unordered thread runs due to: "depending on how your operating system schedules the threads"
    for i in 0..10{
        thread::spawn(move ||{
            println!("Hello world! thread number : {}",i);
            thread::sleep(Duration::from_millis(1));
        });
    }

    // Testing thread order : solution using join will wait for its thread to finish
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    // Testing thread::Result by using join 
    let mut balance: i32 = 100; 
    let result = thread::spawn(move || ->i32{
        for i in 0..10{
            if balance > 0 {
                balance = balance - 10;
                println!("Person number: {} has withdrawn! new balance is: {}",i+1,balance);
            }else{
                balance = 0;
            }
        }
        return balance;
    });

    // Remember: "Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates"
    balance = result.join().unwrap();
    println!("The current bank balance is: {:?}",balance);

    // Adding thread to a vector and call it 'later'
    let mut threads = Vec::new();
    threads.push(move || {
        println!("Some thread started!");
    });
    thread::spawn(threads[0]);
}