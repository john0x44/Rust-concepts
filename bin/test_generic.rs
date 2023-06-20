/*
	Name: test_generic
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing and implementing generics
*/

// Declare a generic struct
struct MyStruct<T>{
    number: T,
}

// Generic multiply function
fn multiply<T>(a: T, b: T) -> T where T: std::ops::Mul<Output = T>,{
    return a * b;
}

// Generic add function 
fn add<T>(a: T, b: T) -> T where T: std::ops::Add<Output = T>,{
    return a + b;
}

// Generic sub function 
fn sub<T>(a: T, b: T) -> T where T: std::ops::Sub<Output = T>,{
    return a - b;
}

// Main test driver
fn main(){
    let some_struct: MyStruct<f64> = MyStruct{ number: 32.32};
    println!("{}",some_struct.number);
    
    println!("{:?}",multiply(20.0,2.0));
    println!("{:?}",add(20.2,2.2));
    println!("{:?}",sub(20.2,2.2));
}