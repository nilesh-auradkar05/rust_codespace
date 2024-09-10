/* statements: almost always end with a semicolon. they do not return a value.
   Eg: variable declarations: let x = 5;
       function definitions: fn add() {}
       control flow statements: if {} else {}, loop {}, while condition {}, for {}
*/

fn add(a: i32, b: i32) -> i32{
    a + b
} 

fn main(){
    
    functions_in_rust("Kratos", 25, 175.5);
    
    // expression: it evaluates the last statement automatically and returns the value.
    let _x = {
        let price: i32 = 10;
        let qty: i32 = 5;
        price * qty
    };

    println!("_x = {}", _x);

    // function call and printing the return value
    let sum: i32 = add(10, 20);
    println!("Addition is : {}", sum);
    println!("Value from function is : {}", add(4, 6));
}

fn functions_in_rust(name: &str, age: u8, height: f32){
    // rust is primarily an expression based language.
    //functions return values.
    // rust support hoisting. hoisting is the process of moving function declarations to the top of the scope.
    // expression: a block of code that returns a value.
    // statement: a block of code that does not return a value.

    println!("My name is {}, my age is {} and my height is {}", name, age, height);
}

