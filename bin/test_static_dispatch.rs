/*
	Name: test_static_dispatch
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing 'static dispatching' on trait objects 
				See file 'test_dynamic_dispatch.rs' for dynamic dispatching!
*/

// Using Rectangle example for 'static dispatching'

// Declare 'Rectangle' struct
struct Rectangle {
    width: f64,
    height: f64,
	shape: String,
}

// Declare abstract base methods
trait Shape {

	// Mutator
    fn area(&self) -> f64;

	// Accessor
	fn get_shape(&self) -> String;
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

	fn get_shape(&self) -> String{
		return self.shape.to_string();
	}
}

// Implement the Shape trait for a Circle
struct Circle {
    radius: f64,
	shape: String,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        return 3.14 * (self.radius * self.radius);
    }

	fn get_shape(&self) -> String{
		return self.shape.to_string();
	}
}

// Calls appropriate method due to 'static dispatch'
fn print_area<T: Shape>(shape: T) {
    println!("Area: {}", shape.area());
	println!("Shape: {}", shape.get_shape())
}

fn main() {
    let rectangle = Rectangle {
        width: 5.0,
        height: 10.0,
		shape: "Rectangle".to_string(),
    };

    let circle = Circle {
        radius: 3.5,
		shape: "Circle".to_string(),
    };

	// Sizes are known at compile time!
    print_area(rectangle);
    print_area(circle);
}
