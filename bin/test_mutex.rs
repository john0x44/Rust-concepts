/*
	Name: test_mutex
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing and implementing mutex or 'shared ownership'
*/

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Main test driver
fn main() {
    // 'balance' will now become have shared ownership and 'mutual exlcusion'
    // Check 'test_arc.rs' for more info on 'Arc' 
    let balance = Arc::new(Mutex::new(100));
    let mut handles = Vec::new();
    for _ in 0..10 {
        let shared_balance = Arc::clone(&balance);
        let handle = thread::spawn(move || {
            let mut balance = shared_balance.lock().unwrap();
            *balance -= 10;
            println!("Withdrew 10 from the account, new balance is {}",balance);
            thread::sleep(Duration::from_millis(10));
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Balance after ALL withdrawals is: {:?}", balance.lock().unwrap());
}