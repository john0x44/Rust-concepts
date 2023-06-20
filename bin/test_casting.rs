/*
	Name: test_casting
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Test casting
*/

// Main test driver
fn main(){
    // Test float to unsigned 32 bit integer
    let this_decimal: f32 = 32.32;
    println!("before casting {}", this_decimal);
    println!("after casting {}", this_decimal as u32);

    // Test char to unsigned 8 bit integer 
    let this_char: char = 'D';
    println!("before casting {}", this_char);
    println!("after casting {}", this_char as u8);

    // Test bool to unsinged 8 bit integer 
    let this_bool: bool = false;
    println!("before casting {}", this_bool);
    println!("after casting {}", this_bool as i8);
}