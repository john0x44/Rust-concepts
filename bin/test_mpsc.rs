/*
	Name: test_mpsc
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing and implementing 'mpsc'; "multiple producer, single consumer"
*/

use std::thread;
use std::sync::mpsc;

// Main test driver
fn main() {

    // Create a channel (sender, reciever)
    let (s, r) = mpsc::channel();

    thread::spawn(move || {
        s.send("Hello from this thread!").unwrap();
    });

    let received = r.recv().unwrap();
    println!("Received message from a thread: {}",  received);
}