/*
	Name: test_overload_operator
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing and implementing overloaded operators
*/

use std::ops::Sub;

// Declare 'MyLetter' struct
#[derive(Clone, Copy, Debug)]
struct MyLetter{
    letter: char,
}

// Implementing subtracting 
// Implment sub for 'MyLetter' or subtracting 'overloaded operator'
impl Sub<MyLetter> for MyLetter {
    type Output = u8;
    
    // Overload subtract
    fn sub(self, rhs: MyLetter) -> Self::Output {
        return (self.letter as u8) - (rhs.letter as u8);
    }
}

// Main test driver
fn main(){
    // ascii 'x' = 120 ; '(' = 40; -> 'P' (80)
    let this_letter: MyLetter = MyLetter{letter: 'x'};
    let this_other_letter: MyLetter = MyLetter{letter: '('};
    println!("{}",(this_letter - this_other_letter) as char);
}