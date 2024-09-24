/* statements: almost always end with a semicolon. they do not return a value.
   Eg: variable declarations: let x = 5;
       function definitions: fn add() {}
       control flow statements: if {} else {}, loop {}, while condition {}, for {}
*/

fn compound_data_types(){
    // array, tuple, string, slice (string slice)

    // array: fixed size, homogeneous (same type)
    let arr: [i32; 5] = [44, 55, 66, 77, 88];

    println!("arr = {:?}", arr);

    // tuple: fixed size, heterogeneous (different types)
    let mixed_tuple: (String, i32, char, bool) = ("Hello".to_string(), 42, 'a', true);
    let mixed_tuple2 = ("Alice", 25, 'F', 1.75, false);

    println!("mixed_tuple with data types defined = {:?}", mixed_tuple);
    println!("mixed_tuple2 without data types defined = {:?}", mixed_tuple2);

    // string: growable, mutable, heap allocated
    let mut my_string: String = String::from("Hey, ");
    my_string.push_str("Kratos!");

    println!("my_string = {}", my_string);

    /* string slice (&str): a reference to a portion of a string store in program. Stored in stack. faster access to memory.
       immutable, fixed length. */
       
    let my_string_slice: &str = &my_string; // prints whole string
    let my_string_slice2: &str = &my_string[0..3]; // prints "Hey"

    println!("my_string_slice = {}", my_string_slice);
    println!("my_string_slice2 = {}", my_string_slice2);
}

fn main(){
    
    compound_data_types();
    
}
