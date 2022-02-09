use std::{fs, str::FromStr, time::Instant};

use word_search_rs::Words;

struct SearchApp {
    url: String,
    words: Words,
    search_for: String,
    iterations: u8,
}

fn get_words() -> Vec<String> {
    let path = "words";
    let words = fs::read_to_string(path).expect("Failed to read words from disk");
    let mut word_vec: Vec<String> = Vec::new();
    for word in words.split("\n") {
        word_vec
            .push(String::from_str(word.clone()).expect("Failed to turn word into String type"));
    }
    return word_vec;
}

fn measure<F>(f: F)
where
    F: Fn() -> u64,
{
    let now = Instant::now();
    f();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn main() {
    let words = get_words();
    let ws = word_search_rs::new(words);
    let simple_clj = || -> u64 { ws.simple_search(String::from_str("query").unwrap()) };
    println!("Linear search");
    measure(simple_clj);
    let binary_clj = || -> u64 { ws.binary_search(String::from_str("query").unwrap()) };
    println!("Binary search");
    measure(binary_clj);
}
