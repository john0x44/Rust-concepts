/*
	Name: test_enums
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing and implementing enumerated types
*/

// Declare an enumerated 'Colors' type
enum Colors{
    Red,
    Blue,
    Green,
}

// Main test driver
fn main(){
    let color: Colors = Colors::Red;
    match color{
        Colors::Red => println!("you chosen an available color! which is Red"),
        Colors::Blue => println!("you chosen an available color! which is Blue"),
        Colors::Green => println!("you chosen an available color! which is Green"),
        _ => println!("This color is not available sorry!"),
    }
}