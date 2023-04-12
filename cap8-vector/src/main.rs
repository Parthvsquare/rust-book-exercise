use std::collections::HashMap;

fn main() {
    let mut v = vec![12, 13, 34];
    // let third = &v[2];
    // let third: Option<&i32> = &v.get(2);

    for i in &mut v {
        *i = *i + 50;

        *i += 50;
        println!("These are in vector, {}", i)
    }

    println!("Hello, world!");
    string_string();
    string_internal_rep();
    hash_hash();
    advance_hash();
    update_hash();
    check_number_of_times("test noTest test");
    update_hash_into_iter();
}

fn string_string() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + &s2 + &s3;
    // add operator will always take ownership of the 1st string
    // let for_s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}

fn string_internal_rep() {
    // let len = String::from("Hola").len();
    let russian = String::from("привет");

    let string_slice = &russian[0..4];
    println!("{}", string_slice);
}

fn hash_hash() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 20);
}

fn advance_hash() {
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_score = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_score.iter()).collect();
    println!("{:?}", initial_score);
    println!("{:?}", scores);
}

fn update_hash() {
    let teams = vec!["Blue", "Red"];
    let initial_score = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.iter().zip(initial_score.iter()).collect();
    // let red = String::from("Red");

    let update_score = scores.entry(&"Red").or_insert(&40);
    println!("{:?}", update_score);
}

fn update_hash_into_iter() {
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_score = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_score.iter()).collect();

    let update_score = scores.entry(String::from("Red")).or_insert(&40);
    println!("{:?}", update_score);
}
fn check_number_of_times(input: &str) {
    let mut map = HashMap::new();
    for word in input.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map)
}

// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
fn exercise_1() {}

// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
fn exercise_2() {}

// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
fn exercise_3() {}
