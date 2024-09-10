/* 
    - Enums
        - Enums allow you to define a type by enumerating its possible variants.
        - 
*/

fn main() {

    // enum declaration

    enum IpAddrKind {
        V4,
        V6,
    }

    // enum instance
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // enum with data
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // enum with multiple types
    enum IpAddress {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddress::V4(127, 0, 0, 1);
    let loopback = IpAddress::V6(String::from("::1"));

    // Option Enum
    
}