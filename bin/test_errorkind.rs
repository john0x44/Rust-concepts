/*
	Name: test_errorkind
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing and implementing ErrorKind
*/

use std::io::ErrorKind;
use std::fs::File;

// Main test driver 
fn main(){
    // Initialize file path
    let file_path = r"test_file.txt";

    // Only for reading, File::open
    let file_read = File::open(file_path);

    // Handle some errors 
    let file_troubleshoot = match file_read{
        Ok(ref some_file) => println!("File opened for reading successful!"),
        Err(ref error) => match error.kind(){
            ErrorKind::NotFound => println!("Did not find file! are you sure '{}' is the valid file location?",file_path),
            _other_error => panic!(),
        }
    };
}