/* statements: almost always end with a semicolon. they do not return a value.
   Eg: variable declarations: let x = 5;
       function definitions: fn add() {}
       control flow statements: if {} else {}, loop {}, while condition {}, for {}
*/

fn main(){
    primitive_data_types();
    compound_data_types();
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
