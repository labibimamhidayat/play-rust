// 📝 Exercise 2: Count Vowels in a String
// Write a function that counts the number of vowels (a, e, i, o, u) in a string.

// ✅ Expected Output:

// rust
// Copy
// Edit
// "Rust is great!" -> 4
// ✅ Hint:

// Use .chars().filter(|c| "aeiouAEIOU".contains(*c)).count().


pub fn count_number_of_vowels_chars() {
    let lorem = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
    let list_of_vowels = ['a', 'i', 'u', 'e', 'o', 'A', 'I', 'U', 'E', 'O'];
    let total_vowels: i32 = lorem.chars().enumerate().fold(0, |accum, (_index, current)| {
        if list_of_vowels.contains(&current) {
            println!("this, before {}, after {}, and current char {}", accum, accum + 1, current);
            return accum + 1;
        }
        return  accum;
    });

    let total_vowels2: usize = lorem.chars().filter(|c|{
        list_of_vowels.contains(c)
    }).count();

    println!("total vowels of '{}' = {} & {}", lorem, total_vowels, total_vowels2)
}



// 📝 Exercise 3: Convert a Sentence to Title Case
// Write a function that capitalizes the first letter of each word in a sentence.

// ✅ Expected Output:

// rust
// Copy
// Edit
// "hello rust world" -> "Hello Rust World"
// ✅ Hint:

// Use .split_whitespace().map(|word| word[..1].to_uppercase() + &word[1..]).

pub fn make_every_words_first_letter_capitalizes(){
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
    let new_text = text.split_whitespace().map(|each_word|{
        let capitalize_word = each_word.chars().enumerate().map(|(index, each_char)|{
            if index == 0 {
                return each_char.to_ascii_uppercase();
            }
            return each_char
        }).collect();
        return capitalize_word;
    }).collect::<Vec<String>>().join(" ");

    println!("this is new Capitilize {}", new_text)
}


