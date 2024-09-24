/*
    - Constants: 
        - Are immutable by default and cannot be changed to mutable.
        - Are declared using the const keyword.
        - Must be assigned a value.
        - Must be explicitly typed. (data type must be specified)
        - can be declared in global or local scope.
*/

fn main(){
    const PI: f32 = 3.14;

    println!("The value of PI is {}", PI);
    println!("The bank name is {}", BANK_NAME);

    shadow_example();
}

const BANK_NAME: &str = "Rust Bank";

/*
    - Shadowing:
        - i.e overriding a variable with a new value.
        - compiler will use the new value instead of the old value.
        - the new value shadows the old value until the end of the scope.
        - shadowing is different from mutability.
            - trying to reassign a new value to below variable x will throw an error.
        - using let, we can perform a few transformations on a value but have the variable immutable after those
            transformations.
        - we're effectively creating a new variable using let keyword. We can change the type of the value but reuse
            the same name.
*/

fn shadow_example(){
    let x = 5;
    println!("The value of x is {}", x);

    // shadowing the above x with a new value
    let x = x + 1;
    println!("The value of x is {}", x); // the new value of x for scope is 6

    {
        let x = x * 2;
        println!("The value of x in inner scope is {}", x); // x = 12
    }

    // The value of x is restored to it's one step earlier value i.e 6
    println!("The value of x is {}", x); // x = 6
}