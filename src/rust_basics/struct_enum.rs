// Exercise 1: Define a Struct and Use It
// Create a struct called Person with the fields:

// name (String)
// age (u32)
// email (String)
// Then, instantiate a Person object and print the details.

// ✅ Example Output:

// yaml
// Copy
// Edit
// Name: Alice, Age: 30, Email: alice@example.com

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug)]
#[allow(dead_code)]
struct Person<'a> {
    name: &'a str,
    age: i32,
    email: &'a str,
}
impl<'a> Person<'a> {
    fn print_detail(&self) {
        println!("{:?}", &self)
    }
}

pub fn show_person() {
    let person = Person {
        name: "labib",
        age: 30,
        email: "labib.imam@gmail.com",
    };
    person.print_detail()
}

// Exercise 2: Add Methods to a Struct
// Modify the Rectangle struct to include a method that calculates perimeter.

// rust
// Copy
// Edit
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect = Rectangle { width: 10, height: 5 };
//     println!("Area: {}", rect.area());
// }

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn perimeter(&self) -> u32 {
        return 2 * (self.width + self.height);
    }
}

pub fn calculates_perimeter() {
    let rect = Rectangle {
        width: 10,
        height: 5,
    };
    println!("Area: {}", rect.area());
    println!("perimeter: {}", rect.perimeter())
}

// Define an enum called Shape with two variants:

// Circle(radius: f64)
// Rectangle(width: f64, height: f64)
// Write a function that calculates the area of any Shape using pattern matching.

// ✅ Example Output:

// yaml
// Copy
// Edit
// Circle area: 78.54
// Rectangle area: 20.0

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

fn calculates_shape(s: &Shape) -> f64 {
    match s {
        Shape::Circle(r) => {
            return 3.14 * r * r;
        }
        Shape::Rectangle(p, l) => {
            return p * l;
        }
    }
}

pub fn check_shape() {
    let shape_circle = Shape::Circle(31.4);
    let shape_rec: Shape = Shape::Rectangle(32.4, 11.2);
    println!(
        "calculated area, circle {}, rectagle {}",
        calculates_shape(&shape_circle),
        calculates_shape(&shape_rec)
    )
}

// Write a function that returns Option<f64> instead of panicking when dividing two numbers.

// rust
// Copy
// Edit
// fn safe_divide(a: f64, b: f64) -> Option<f64> {
//     // Fix this function
// }

// fn main() {
//     let result = safe_divide(10.0, 2.0);

//     match result {
//         Some(value) => println!("Result: {}", value),
//         None => println!("Cannot divide by zero"),
//     }
// }

fn safe_divide(a: f64, b: f64) -> Option<f64> {
    // Fix this function
    if b == 0.0 {
        return None;
    }
    return Some(a / b);
}

pub fn display_save_divided() {
    let result: Option<f64> = safe_divide(10.0, 0.0);

    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("Cannot divide by zero"),
    }
}

// Exercise 5: if let for Handling an Option<T>
// Modify the previous exercise to use if let instead of match.

// ✅ Expected Output: (same as before)

// Result: 5.0
// or

// Cannot divide by zero

pub fn display_save_divided_if_let() {
    let result: Option<f64> = safe_divide(10.0, 0.0);

    if let Some(val) = result {
        println!("Result: {}", val);
        return;
    }
    println!("Cannot divide by zero AKWOAWKOAW");
}

// Exercise 2: Store User Data in a HashMap
// Instead of using a struct, store user data in a HashMap<String, String>.

// ✅ Expected Output:

// css
// Copy
// Edit
// User: {"name": "Alice", "age": "30", "email": "alice@example.com"}
// ✅ Hint:

// Use HashMap<String, String>.
// Insert "name", "age", and "email" as keys.

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: i32,
    email: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Public {
    user: User,
    address: String,
}

pub fn store_struct_to_hash_map() {
    let user: User = User {
        name: String::from("Labib"),
        age: 20,
        email: String::from("hahah@gmail.com"),
    };
    let json_value_struct = serde_json::to_value(&user).unwrap();
    let map: User = serde_json::from_value(json_value_struct.clone()).unwrap();
    let map_hash: HashMap<String, Value> = serde_json::from_value(json_value_struct).unwrap();
    println!("{:?}, {:?}", map, map_hash);
}

pub fn test_child_struct() {
    let public1: Public = Public {
        user: User {
            name: String::from("Labib"),
            age: 20,
            email: String::from("hahah@gmail.com"),
        },
        address: String::from("Jalan"),
    };
    let json_value: Value = serde_json::to_value(&public1).unwrap();
    let hash: HashMap<String, Value> = serde_json::from_value(json_value.clone()).unwrap();
    let mapped: Public = serde_json::from_value(json_value.clone()).unwrap();
    println!("{:?}", json_value);
    println!("wkkwkw{:?}", hash);
    println!("askdaskod, {:?}", mapped);
}


// 📝 Exercise 5: Find a Value in a HashMap Safely
// Write a function that searches for a key in a HashMap and handles missing values safely.

// ✅ Expected Output:

// makefile
// Copy
// Edit
// Name: Alice
// Hobby: Not Found
// ✅ Hint:

// Use .get("key") and handle missing keys with .unwrap_or("Not Found").

pub fn find_key_hash_map() {
    let user: User = User {
        name: String::from("Labib"),
        age: 20,
        email: String::from("hahah@gmail.com"), 
    };
    let json_value_struct = serde_json::to_value(&user).unwrap();
    let map_hash: HashMap<String, Value> = serde_json::from_value(json_value_struct).unwrap();
    let text_err = &Value::String(String::from("error nih"));
    let name = map_hash.get("name").unwrap_or(text_err);
    let not_found_name = map_hash.get("ampas").unwrap_or(text_err);
    println!("{:?}, {:?}", name, not_found_name);
}