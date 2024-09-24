/*  
    - Control Flow statements:
        - Conditional statements: if, else, else if.
            - if {} else {}
            - if {} else if {} else {}
            - if in let statement.
        - Note: type of value in if arm should be same.
        - Flow statements: loop, while, for.

*/

fn conditional_statements(){

    // multiple conditions check using if-else if.
    let x: i32 = 10;
    let y: i32 = -20;
    let z: i32 = 2;

    if x > y && x > z {
        println!("x is greater than y and z");
    } else if y > z && y > x {
        println!("y is greater than x and z");
    } else if z > x && z > y {
        println!("z is greater than x and y");
    } else {
        println!("All are equal!");
    }

    // if in let statement.
    let is_adult = true;
    let age = if is_adult { 18 } else { 17 };
    
    println!("Age: {}", age);

}

/*
    - Loops:
        - loop: infinite loop.
            - loops can be labeled to disambiguate between multiple loops.
            - break, continue. can be used to control the flow of the loop.

        - while: loop until condition is false.
        - for: loop until condition is false.
*/

fn loops_in_rust(){
    let mut count = 0;
    'counting_up: loop{
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 7 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    
    println!("End count = {count}");

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("While loop ended!!!");

    // for loop
    let a: [i32; 5] = [1, -3, 4, -1, -5];
    for i in a{
        println!("{i}");
    }
}

fn main(){
    conditional_statements();
    loops_in_rust();
}