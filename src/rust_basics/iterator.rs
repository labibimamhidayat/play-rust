// Exercise 1: Use iter() to Print All Elements
// Write a function that uses iter() to print each number without modifying the vector.

// ✅ Expected Output:

// Copy
// Edit
// 1
// 2
// 3
// 4
// 5
// ✅ Hint:

// Use for num in numbers.iter() to loop over references.

use std::collections::{HashMap, HashSet};

// use serde::de::value;

pub fn iterate_number() {
    let number = [1, 2, 3, 4, 5];
    for i in number.iter() {
        println!("{}", i)
    }
}

// Use into_iter() to Move Ownership
// Modify the function to use into_iter(), so that the vector cannot be used after iteration.

// ✅ Expected Output:

// Copy
// Edit
// 1
// 2
// 3
// 4
// 5
// ✅ Hint:

// Use for num in numbers.into_iter().
// Try printing numbers after the loop—what happens?

pub fn iterate_number_move_ownership() {
    let number = vec![1, 2, 3, 4, 5];
    let mut new_x: Vec<i32> = Vec::new();
    for i in number.into_iter() {
        new_x.push(i);
    }
    // try to print this shit, ownership has been moved
    // println!("{:?}", number);
    println!("{:?}", new_x);
}

// Exercise 3: Use iter_mut() to Modify Each Element
// Write a function that uses iter_mut() to double each number in a vector.

// ✅ Expected Output:

// csharp
// Copy
// Edit
// [2, 4, 6, 8, 10]
// ✅ Hint:

// Use for num in numbers.iter_mut() { *num *= 2; }.

pub fn iterate_number_modified_it() {
    let mut number = vec![1, 2, 3, 4, 5];
    for i in number.iter_mut() {
        *i = *i * 5;
    }
    // try to print this shit, ownership has been moved
    // println!("{:?}", number);
    println!("{:?}", number);

    let mut text: Vec<String> = vec![
        "wkwk".to_string(),
        "labib".to_string(),
        "ahhaha".to_string(),
    ];
    println!("beforeee, {:?}", text);

    for each_text in text.iter_mut() {
        *each_text = format!("{} -> {}", each_text, "heheh")
    }
    println!("after => {:?}", text);
}

// Exercise 4: Convert a Vector of Strings to Uppercase Using map()
// Use .map() with an iterator to convert all elements in a vector to uppercase.

// ✅ Expected Output:

// css
// Copy
// Edit
// ["HELLO", "WORLD"]
// ✅ Hint:

// Use .map(|s| s.to_uppercase()).
// Use .collect::<Vec<String>>() to collect results.

pub fn modified_using_map() {
    let text: Vec<String> = vec![
        "wkwk".to_string(),
        "labib".to_string(),
        "ahhaha".to_string(),
    ];
    println!("beforeee, {:?}", text);

    let new_text: Vec<String> = text
        .iter()
        .map(|each_text: &String| each_text.to_uppercase())
        .collect();
    println!("after => {:?}", new_text);

    let b = |x: i32, b: &str| println!("{}, {}", x, b);

    fn a() {
        println!("asdkasooko")
    }

    b(1, "ksks");
    a();
}

// Exercise 5: Filter Even Numbers from a Vector Using filter()
// Write a function that keeps only even numbers from a vector.

// ✅ Expected Output:

// csharp
// Copy
// Edit
// [2, 4, 6]
// ✅ Hint:

// Use .filter(|&x| x % 2 == 0).

pub fn filter_not_palindrome() {
    let list_of_words = vec![
        "noon", "rust", "level", "racecar", "madam", "hello", "coding", "radar", "world",
    ];
    let list_of_words_string: Vec<String> =
        list_of_words.into_iter().map(|x| x.to_string()).collect();
    println!("{:?}", list_of_words_string);
    let filtered_words_string: Vec<String> = list_of_words_string
        .into_iter()
        .filter(|x: &String| {
            let flipped: String = x.chars().rev().collect();
            if *x == flipped {
                return true;
            }
            return false;
        })
        .collect();
    println!("{:?}", filtered_words_string);
}

// Exercise 5: Remove Duplicates from a Vector
// Write a function that removes duplicates from a vector.

// ✅ Expected Output:

// css
// Copy
// Edit
// [1, 2, 2, 3, 4, 4] -> [1, 2, 3, 4]
// ✅ Hint:

// Use .into_iter().collect::<HashSet<_>>().

pub fn remove_duplicate() {
    let list_vec = vec![1, 2, 2, 3, 4, 4];
    let listed: HashSet<i32> = list_vec.into_iter().collect();
    println!("asd {:?}", listed);
}

// Exercise 6: Find the Most Frequent Element in a Vector
// Write a function that finds the most frequently occurring element in a vector.

// ✅ Expected Output:

// csharp
// Copy
// Edit
// [1, 3, 3, 2, 1, 3, 2, 2,4,3,2,1,1,10,1,2,3,9,0,0,1] -> 3
// ✅ Hint:

// Use HashMap<K, usize> to count occurrences.

pub fn find_most_freq_number() {
    let list_vec: Vec<i32> = vec![
        1, 3, 3, 2, 1, 3, 2, 2, 4, 3, 2, 1, 1, 10, 1, 2, 3, 9, 0, 0, 1,
    ];
    let mut list_of_hash = HashMap::new();
    // let unique_vec: HashMap<usize, i32> = list_vec.into_iter().enumerate().collect();
    for val in list_vec.into_iter() {
        *list_of_hash.entry(val).or_insert(0) += 1
    }
    println!("list hash {:?}", list_of_hash);
    let (key, _) = list_of_hash.iter().max_by_key(|&(_, count)| count).unwrap();
    println!("yg terbanyak {:?}", key);
}

// Exercise 7: Merge Two HashMap<String, i32>
// Write a function that merges two hashmaps, summing values for duplicate keys.

// ✅ Expected Output:

// rust
// Copy
// Edit
// {"apple": 5, "banana": 2} + {"apple": 3, "orange": 4} -> {"apple": 8, "banana": 2, "orange": 4}
// ✅ Hint:

// Loop through hashmap2 and .entry().or_insert() + value.

pub fn join_two_hash_map() {
    let mut hash1 = HashMap::new();
    let mut hash2 = HashMap::new();
    hash1.insert("apple", 5);
    hash1.insert("banana", 2);
    hash2.insert("apple", 3);
    hash2.insert("orange", 4);

    for (key, value) in hash1.iter() {
        println!("key {} value {}", key, value);
        *hash2.entry(key).or_insert(0) += *value
    }

    println!("{:?}", hash2);
}
