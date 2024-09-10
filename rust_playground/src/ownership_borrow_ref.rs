/* statements: almost always end with a semicolon. they do not return a value.
   Eg: variable declarations: let x = 5;
       function definitions: fn add() {}
       control flow statements: if {} else {}, loop {}, while condition {}, for {}
*/

/* Ownership:
    - Every value has a single owner [every variable has one value, and it is it's sole owner].
    
    Rules:
        - Each value in rust has a variable that's it's owner.
        - There can only be one owner at a time.
        - When the owner goes out of scope, the value is dropped.
*/

// Ownership Rules implementation
fn main(){
    // Each value in Rust has a variable that's it's owner.
    // Example 1:
    
    let s1: String = String::from("Kratos"); // s1 is the sole owner of the string.
    let s1_len = string_length(&s1); // &s1 is a reference to the string and not the actual value of the string.

    println!("Length of s1 string {} is {}", s1, s1_len);

    // There can only be one owner at a time.
    // Example 1:
    /*
    let s2 = s1;
    println!("s2 = {}", s1); // This will throw an error because s1 is no longer the owner of the string.
    */
    
    reference_borrowing_example();

}

// When the owner goes out of scope, the value is dropped.
//Example 1

fn string_length(s: &String) -> usize{
    s.len()
}

/*
fn s1_scope_lost(s: &String){
    println!("s1 = {}", s); // This will throw an error because s1 is out of scope and cannot be referenced.
}
*/
/*
    - Reference and borrowing:
        - Safety and Performance:
            - Safety:
                - Rust does not allow for dangling pointers, null pointer dereferencing, etc.
            - Performance:
                - Rust does not allow for reference cycles, which can cause memory leaks in other languages.
        
        - We create references by borrowing the value from the original owner of the value.
        - References: Enables to borrow the value without taking ownership of it.
        - References: immutable and mutable types.
        - To create a reference variable needs to preceed with & symbol.
*/

// Reference and borrowing example
fn reference_borrowing_example(){
    let _x = 5;
    let _y = &_x; // _y is a reference to _x.

    println!("value of _x = {}, value of _y = {}", _x, _y);

    // mutable reference
    let mut _z = 10;
    let _z_ref = &mut _z; // _z_ref is a mutable reference to _z.

    *_z_ref += 1;
    *_z_ref -= 2;
    println!("value of _z_ref = {}", _z_ref);
}
