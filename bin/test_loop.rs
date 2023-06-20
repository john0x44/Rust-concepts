/*
	Name: test_break
	Copyright: 2023
	Author: John
	Date: 17/06/23 19:39
	Description: Testing and implementing break
*/

//Main test driver
fn main(){
    let mut times = 0; 
    loop {
        println!("I said hello world {} times!",times+1);
        times = times + 1;
        if times == 10 {break;}
    }
}
