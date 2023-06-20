/*
	Name: test_from
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing and implementing 'from' 
*/

// Declare a 'AddNum' struct
#[derive(Debug)]
struct AddNum{
    number: i32,
}

// Implement from for 'AddNum'
impl From<i32> for AddNum{
    fn from(number: i32) -> Self {
        Self { number: number + 1}
    }
}

// Main test driver
fn main(){
    let new_number = AddNum::from(22);
    println!("{:?}",new_number.number);
}