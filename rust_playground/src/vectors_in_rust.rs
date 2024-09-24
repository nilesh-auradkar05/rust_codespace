fn main() {

    // initializing a vector
    let _v: Vec<i32> = Vec::new();

    // creating a vector using vec! macro
    let mut new_vec = vec![1,2,3,4,5];

    new_vec.push(6);
    new_vec.push(7);
    new_vec.push(8);
    new_vec.push(9);
    new_vec.push(10);

    println!("{:?}", new_vec);

    // accessing elements of a vector
    let third_element: &i32 = &new_vec[2];

    println!("The third element is {third_element}");

    // using get method to access elements of a vector
    let fourth_element: Option<&i32> = new_vec.get(3);

    match fourth_element {
        Some(result) => println!("The fourth element is {result}"),
        None => println!("Index error: out of bounds!")
    }

    // accessing out of bounds element
    let out_of_bound_element: Option<&i32> = new_vec.get(10);

    match out_of_bound_element {
        Some(vec_element) => println!("The element is {vec_element}"),
        None => println!("Index error: out of bounds!")
    }

}