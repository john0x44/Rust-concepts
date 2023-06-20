/*
	Name: test_file_handling
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing and implementing file handling
*/

use std::io::ErrorKind;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::fs;
use std::fs::OpenOptions;

// Return content of file!
fn return_file_content(mut file: &File) -> String{
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();
    return file_content;
}

// Report contents of a specified file path!
fn report_file_content(file_path: &str){
    let file_read = File::open(file_path);
    println!("{}",return_file_content(&file_read.unwrap()));
}

// Main test driver 
fn main(){
    // Initialize file path
    let file_path = r"C:\Users\onemy\OneDrive\Desktop\hello\src\bin\test_file.txt";

    // Only for reading, File::open
    let file_read = File::open(file_path);

    // Handle some errors 
    let file_troubleshoot = match file_read{
        Ok(ref some_file) => println!("File opened for reading successful!"),
        Err(ref error) => match error.kind(){
            ErrorKind::NotFound => println!("Did not find file! are you sure this is the valid location!"),
            _other_error => panic!(),
        }
    };
    
    // Send file object to function
    println!("{}",return_file_content(&file_read.unwrap()));

    // Create & write to file 
    let new_file_path = "new_test_file.txt";
    let mut new_file = File::create(new_file_path).expect("something went wrong when creating new file!");
    new_file.write("This is a newly created text file!".as_bytes());

    // Report file content 
    report_file_content(&new_file_path);

    // Append to file! 
    // OpenOptions::wri
    te "If the file already exists, any write calls on it will overwrite its contents, without truncating it."
    let mut data_file = OpenOptions::new().append(true).open(new_file_path).expect("something went wrong when opening file!");

    // Write to a file
    data_file.write("\nThis is another line!".as_bytes());
    
    // Report the new file contents! 
    report_file_content(&new_file_path);

    // Remove a file 
    fs::remove_file(new_file_path);
}