/* tuple structs
    - they do not have named fields
*/

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/* unit like structs
    - they have no fields
    - Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data 
      that you want to store in the type itself.
    - they are useful for generics.
*/

struct AlwaysEqual;

fn main() {
    struct User {
        is_active: bool,
        username: String,
        email: String,
        sign_in_count: u32,
    }

    struct Book {
        title: String,
        author: String,
        isbn: u32,
        pages: u32,
        is_available: bool,
    }

    let mut root_user: User = User {
        is_active: true,
        username: "Raymond".to_string(),
        email: "rayred@gmail.com".to_string(),
        sign_in_count: 5,
    };

    println!("Username for {} is {} and status is {}", root_user.email, root_user.username, root_user.is_active);

    root_user.username = "RayRed".to_string();
    println!("Updated username for {} is {}", root_user.email, root_user.username);

    // function returning a struct object
    fn build_user(email: String, username: String) -> User {
        User {
            is_active: true,
            username,
            email,
            sign_in_count: 3,
        }
    }

    let new_user: User = build_user(String::from("testuser@gmail.com"), String::from("Test User"));

    println!("Function returned struct user details: {} {} {} {}", new_user.username, new_user.email, new_user.is_active, new_user.sign_in_count);


    // creating a new instance from an existing instance
    let root_user_2: User = User {
        username: String::from("Elizabeth"),
        email: String::from("elizabeth@gmail.com"),
        ..root_user
    };

    println!("Root user 2 details: {} {} {} {}", root_user_2.username, root_user_2.email, root_user_2.is_active, root_user_2.sign_in_count);

    
    // creating instance of tuple structs from global scope
    let black: Color = Color(0, 0, 0);
    let black_origin: Point = Point(0, 0, 0);

    let white: Color = Color(255, 255, 255);
    let white_origin: Point = Point(255, 255, 255);

    println!("Black color: {} {} {}", black.0, black.1, black.2);
    println!("Black origin: {} {} {}", black_origin.0, black_origin.1, black_origin.2);
    println!("White color: {} {} {}", white.0, white.1, white.2);
    println!("White origin: {} {} {}", white_origin.0, white_origin.1, white_origin.2);


    // calling unit like structs from global scope
    let subject: AlwaysEqual = AlwaysEqual;
}




