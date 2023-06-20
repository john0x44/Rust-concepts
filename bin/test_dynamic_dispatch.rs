/*
	Name: test_dynamic_dispatch
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing 'dynamic dispatching' on trait objects 
*/

// Declare 'Animal' trait
trait Animal {
    fn make_noise(&self);
}

// Declare structs
struct Dog;
struct Cat;

// Implement Animal for 'Dog' struct
impl Animal for Dog {
    fn make_noise(&self) {
        println!("Dog goes 'woof'");
    }
}

// Implement Animal for 'Cat' struct
impl Animal for Cat {
    fn make_noise(&self) {
        println!("Cat goes 'meow'");
    }
}

// Calls appropriate method due to 'dynamic dispatch'
fn make_noise(animal: &dyn Animal) {
    animal.make_noise();
}

// Main test driver
fn main() {
    let dog: Dog = Dog;
    let cat: Cat = Cat;
    // &dyn is a fat pointer that contains a pointer to vtable and type in heap!
    // Size of fat pointer is known at compile time
    let animals: Vec<&dyn Animal> = vec![&dog, &cat];
    
    make_noise(animals[0]);

}
