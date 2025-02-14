use crate::rust_basics::variable_play;
// Exercise 1: Fix the Borrowing Error
// The following code has a borrowing error. Can you fix it?

// fn main() {
//     let mut message = String::from("Hello");

//     let r1 = &message;
//     let r2 = &message;
//     let r3 = &mut message; // ❌ ERROR: Mutable borrow after immutable borrows

//     println!("{}, {}", r1, r2);
// }


pub fn fix_borrowing_error() {
    let mut message = String::from("Hello");

    let r1 = &message;
    let r2 = &message;
    println!("{}, {}", r1, r2);
    let r3 = &mut message; // ❌ ERROR: Mutable borrow after immutable borrows
    println!("{}", r3);
    variable_play::hello_world();

}


// Exercise 2: Return a Borrowed String Slice (&str)
// Modify the function so that it returns a borrowed &str instead of String.

// rust
// fn first_word(s: &String) -> String { // ❌ Avoid returning `String`
//     s[..4].to_string()
// }

// fn main() {
//     let text = String::from("Rustacean");
//     let word = first_word(&text);
//     println!("First word: {}", word);
// }

fn first_word(s: &String) -> &str { // ❌ Avoid returning `String`
    return &s[..4]
}

pub fn return_borrowed_string_slice() {
    let text = String::from("Rustacean");
    let word = first_word(&text);
    println!("First word: {}", word);
}

// Exercise 3: Refactor &mut into a Pure Function
// Convert this function so that it does not modify the input but instead returns a new String.

// fn add_exclamation(s: &mut String) { // ❌ Uses &mut
//     s.push_str("!");
// }

// fn main() {
//     let mut text = String::from("Hello");
//     add_exclamation(&mut text);
//     println!("{}", text);
// }


fn add_exclamation(s: &str) -> String { // ❌ Uses &mut
    let mut y: String = s.to_owned();
    y.push_str("!");
    return y
}

pub fn pure_function_only() {
    let text = String::from("Hello");
    let new_text = add_exclamation(&text);
    println!("{}, {}", text, new_text);
}

// Exercise 4: Find the Largest Number Without Moving Ownership
// Fix the function so it returns a reference to the largest number instead of moving ownership.

// fn largest(numbers: Vec<i32>) -> i32 { // ❌ Moves ownership of `numbers`
//     let mut max = numbers[0];
//     for &num in &numbers {
//         if num > max {
//             max = num;
//         }
//     }
//     max
// }

// fn main() {
//     let nums = vec![10, 20, 5, 30, 15];
//     let max = largest(nums); // ❌ `nums` is moved and no longer usable
//     println!("Largest: {}", max);
// }

fn largest(numbers: &Vec<i32>) -> &i32 { // ❌ Moves ownership of `numbers`
    let mut max = &numbers[0];
    for num in numbers {
        if num > max {
            max = num;
        }
    }
    return max
}

pub fn find_largest_number() {
    let nums = vec![10, 20, 5, 30, 15];
    let max: &i32 = largest(&nums);
    println!("Largest: {}", max);
}


// Exercise 5: Fix the Lifetime Issue in a Struct
// ❌ Original Code (Lifetime Error)
// rust
// Copy
// Edit
// struct Data {
//     value: &str, // ❌ ERROR: Missing lifetime annotation
// }

// fn main() {
//     let text = String::from("Hello");
//     let data = Data { value: &text };

//     println!("{}", data.value);
// }


struct Data<'a> { // ✅ Add lifetime `'a`
    value: &'a str, // ✅ Reference with lifetime
}

pub fn lifetime_issue() {
    let text = String::from("Hello");
    let data = Data { value: &text }; // ✅ Now it works

    println!("{}", data.value);
}


// Exercise 2: Find the Longest Word (Return &str)
// Write a function that takes two string slices and returns the longest one.

// rust
// Copy
// Edit
// fn longest_word(s1: &str, s2: &str) -> &str { // ❌ ERROR: Missing lifetime
//     if s1.len() > s2.len() {
//         s1
//     } else {
//         s2
//     }
// }

// fn main() {
//     let word1 = String::from("Rust");
//     let word2 = String::from("Programming");

//     let result = longest_word(&word1, &word2);
//     println!("Longest word: {}", result);
// }


fn longest_word<'a>(s1: &'a str, s2: &'a str) -> &'a str { // ❌ ERROR: Missing lifetime
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

pub fn find_longest_word() {
    let word1 = String::from("Rust");
    let word2 = String::from("Programming");

    let result = longest_word(&word1, &word2);
    println!("Longest word: {}", result);
}


// Exercise 3: Convert &mut Function to Pure Function
// Refactor the following function so it does not modify the input directly but instead returns a new String.

// rust
// Copy
// Edit
// fn add_prefix(s: &mut String) { // ❌ Uses &mut (side effect)
//     s.insert_str(0, "Rust-");
// }

// fn main() {
//     let mut text = String::from("Lang");
//     add_prefix(&mut text);
//     println!("{}", text);
// }

fn add_prefix(s: &str) -> String { // ❌ Uses &mut (side effect)
    return format!("{}{}","Rust-",s)
}

pub fn change_mut() {
    let mut text = String::from("Lang");
    text = add_prefix(&text);
    println!("{}", text);
}

// Exercise 4: Fix the Vec Borrowing Issue
// The following function tries to return a reference to an element in a Vec, but it has a borrowing issue. Fix it.

// fn first_element(vec: &mut Vec<i32>) -> &i32 {
//     vec.push(100); // ❌ ERROR: Mutable borrow while returning an immutable reference
//     &vec[0] // ❌ This reference might be invalid
// }

// fn main() {
//     let mut numbers = vec![1, 2, 3];
//     let first = first_element(&mut numbers);
//     println!("{}", first);
// }

fn first_element(vec: &mut Vec<i32>) -> &i32 {
    vec.push(100); // ❌ ERROR: Mutable borrow while returning an immutable reference
    &vec[0] // ❌ This reference might be invalid
}

pub fn fix_vec() {
    let mut numbers: Vec<i32> = vec![1, 2, 3];
    let first = first_element(&mut numbers);
    println!("{}", first);
}