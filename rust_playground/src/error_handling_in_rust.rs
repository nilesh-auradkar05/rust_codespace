/* 
    - Error Handling
        - Rust groups errors into two major categories: recoverable and unrecoverable errors.
        - For recoverable errors, the `Result` type is used.
        - For unrecoverable errors, the `panic!` macro is used.

    - Unrecoverable Errors
        - Unrecoverable errors occur when something impossible happens.
 
    - 2 methods to handle errors: Option<T> and Result<T, E> enums.
        - Option<T>{
            Some(T),
            None,
        }

        - Result<T, E>{
            Ok(T),
            Err(E),
        }
*/

// Approach 1: Option<T>
fn divide_option(numerator: i32, denominator: i32) -> Option<i32> {
    if denominator == 0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

// Approach 2: Result<T, E>
fn divide_result(numerator: i32, denominator: i32) -> Result<i32, String> {
    if denominator == 0 {
        Err("Cannot divide by Zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {

    // invoking divide function for divide by 0 error
    let result = divide_option(6, 0);
    match result {
        Some(x) => println!("Divison result: {x}"),
        None => println!("Error: Division by zero is not allowed.")
    }

    // invoking divide function for a valid division
    let valid_division = divide_option(6, 3);
    match valid_division {
        Some(y) => println!("Division result: {y}"),
        None => println!("Error: Division by zero is not allowed.")
    }

    // invoking divide_result function for divide by 0 error
    let result_error = divide_result(10, 0);
    match result_error {
        Ok(x) => println!("Division result: {x}"),
        Err(e) => println!("Error: {e}")
    }

    // invoking divide_result function for a valid division
    let valid_result = divide_result(10, 2);
    match valid_result {
        Ok(div_result) => println!("Division result: {div_result}"),
        Err(err) => println!("Error: {err}")
    }
    

}
