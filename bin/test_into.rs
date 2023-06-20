/*
	Name: test_into
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing and implementing 'into'
*/

use std::ops::Sub;

// Declare a 'Celsius' struct
#[derive(Clone, Copy, Debug)]
struct Celsius{
    temp: f64,
}

// Declare 'Fahrenheit' struct
#[derive(Clone, Copy, Debug)]
struct Fahrenheit{
    temp: f64,
}

// Implment sub for Fahrenheit or 'overloaded operator'
impl Sub<Fahrenheit> for Fahrenheit {
    type Output = f64;

    fn sub(self, other: Fahrenheit) -> Self::Output {
        self.temp - other.temp
    }
}

// Implement from for 'AddNum'
impl From<Fahrenheit> for Celsius{
    fn from(fahrenheit: Fahrenheit) -> Self {
        Celsius { temp: ((fahrenheit.temp - 32.0)*5.0) / 9.0}
    }
}

// Main test driver
fn main(){
    let new_temp: Fahrenheit = Fahrenheit{temp: 32.0};
    let new_ctemp: Celsius = new_temp.into();
    println!("The temperature in F: {} is converted to celsius: {:.2}",new_temp.temp,new_ctemp.temp);
}