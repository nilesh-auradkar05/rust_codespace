/* statements: almost always end with a semicolon. they do not return a value.
   Eg: variable declarations: let x = 5;
       function definitions: fn add() {}
       control flow statements: if {} else {}, loop {}, while condition {}, for {}
*/


fn primitive_data_types(){
    // integer, float, bool, char
    
    // signed integer: i8, i16, i32, i64, i128.
    // unsigned integer: u8, u16, u32, u64, u128.
    let x: i32 = -10; // range: -2^31 to 2^31
    let y: i64 = 100000; // range: -2^63 to 2^63

    println!("x = {}, y = {}", x, y);

    // float: f32, f64
    let pi: f32 = 3.14;

    println!("pi = {}", pi);

    // bool
    let is_active: bool = true;
    
    println!("is_active = {}", is_active);

    // char
    let a: char = 'a';

    println!("a = {}", a);
}

fn main(){
    primitive_data_types();
    
}
