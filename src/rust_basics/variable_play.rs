use std::any::type_name;

fn print_type<T>(_: &T) {
    println!("Type: {}", type_name::<T>());
}

pub fn see_type<T>(x: &T) {
    print_type(x);
}

pub fn hello_world() {
    println!("hellow world");
}

pub fn play_with_tupple() {
    let p = (1, "labib", String::from("zaza"));
    see_type(&p);
    println!("this is inside {}", p.2);
    let w = p.0 + 1;
    println!("this is from tuple number 1 add 2 = {}", w);
    println!("{:#?}", p);
}

pub fn play_with_array() {
    let array_me: [&str; 3] = ["haha", "labib", "asdklopaskodas"];
    see_type(&array_me);
    // create new heap, bad for performance but leave no side effect 
    let w: Vec<String> = array_me.iter().map(|x| {format!("kwkwkw anmpas {}", x.to_string())}).collect();
    dbg!(array_me);
    see_type(&w);
    dbg!(w);
    let mut time = [1,2,3,4];
    time.iter_mut().for_each(|each: &mut i32| {
        let p = *each;
        if *each % 2 == 0 {
           return *each = *each + 3 * (p / 2)
        }
        return *each = *each + 5 * (p / 4)
    });
    see_type(&time);
    dbg!(time);
}

pub fn play_with_string() {
    let str = String::from("ahahha");
    let b: String = str + "lwlw";
    println!("{:?}", b)
}


// 1 challange fix the error 

// fn error_func() {
//     let mut x = 10;
//     println!("x before: {}", x);

//     x = "hello"; // ❌ Error: Mismatched types
//     println!("x after: {}", x);
// }
// answer
pub fn fix_func() {
    let mut x: i32 = 10;
    println!("x before: {}", x);

    x = 1;
    println!("x after: {}", x);
}

// 2 challnge Write a program that checks if a given number is even or odd.

pub fn check_even_or_odd(number: &i32) -> &'static str {
    if number % 2 == 0 {
        "even"
    } else {
        "odd"
    }
}

// 3 sum number 

pub fn sum_number(n_number: i32) -> i32 {
    return (1..n_number).sum();
}

// Exercise 4: Function with &str and String

// fn main() {
//     let name = "Alice";
//     let greeting = greet(name);
//     println!("{}", greeting);
// }

// expected : Hello, Alice

pub fn greeting_func(name: &str) -> String {
    return format!("Hello, {}", name);
}
