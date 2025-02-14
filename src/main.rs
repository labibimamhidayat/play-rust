
use play_rust::rust_basics::{variable_play, borrowing_ownership, struct_enum, iterator, string_slices};
// call direct function inside file variable_play inside folder rust_basics
use variable_play::hello_world;

#[allow(dead_code)]
fn rust_basics() {
    // let x = 1;
    // let b = "hahaha";
    // let c = String::from("Labib Ganteng");
    // let mut w: String = String::from("labib");
    // w = w.chars().rev().collect();
    // let za = true;
    // variable_play::see_type(&x);
    // variable_play::see_type(&b);
    // variable_play::see_type(&c);
    // variable_play::see_type(&w);
    // println!("this flipped {}", &w);
    hello_world();
    // variable_play::see_type(&za);
    // variable_play::play_with_tupple();
    variable_play::play_with_array();
    variable_play::fix_func();
    println!("what is this? {}", variable_play::check_even_or_odd(&2));
    println!("what is this? {}", variable_play::check_even_or_odd(&3));
    dbg!(variable_play::sum_number(10));
    let greetings = variable_play::greeting_func("labib");
    dbg!(&greetings);
}

#[allow(dead_code)]
fn borrowing_ownership() {
    borrowing_ownership::fix_borrowing_error();
    borrowing_ownership::return_borrowed_string_slice();
    borrowing_ownership::pure_function_only();
    borrowing_ownership::find_largest_number();
    borrowing_ownership::find_longest_word();
    borrowing_ownership::change_mut();
    borrowing_ownership::fix_vec();
}

#[allow(dead_code)]
fn struct_enum() {
    struct_enum::show_person();
    struct_enum::calculates_perimeter();
    struct_enum::check_shape();
    struct_enum::display_save_divided();
    struct_enum::display_save_divided_if_let();
    struct_enum::store_struct_to_hash_map();
    struct_enum::test_child_struct();
    struct_enum::find_key_hash_map();
}

#[allow(dead_code)]
fn iteration() {
    // iterator::iterate_number();
    // iterator::iterate_number_move_ownership();
    // iterator::iterate_number_modified_it();
    // iterator::modified_using_map();
    // iterator::filter_not_palindrome();
    iterator::remove_duplicate();
    iterator::join_two_hash_map();
    iterator::find_most_freq_number();
}

#[allow(dead_code)]
fn manipulated_string() {
    string_slices::count_number_of_vowels_chars();
    string_slices::make_every_words_first_letter_capitalizes();
}

fn main() {
    // rust_basics();
    // borrowing_ownership();
    // struct_enum();
    iteration();
    // manipulated_string();
}
