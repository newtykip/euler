/*
Problem 42 - Coded triangle numbers

The  n  th  term of the sequence of triangle numbers is given by,  t n   = ½ n ( n +1); so the first ten triangle numbers are:
1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
By converting each letter in a word to a number corresponding to its alphabetical position and adding these values we form a word value. For example, the word value for SKY is 19 + 11 + 25 = 55 =  t  10 . If the word value is a triangle number then we shall call the word a triangle word.
Using  words.txt  (right click and 'Save Link/Target As...'), a 16K text file containing nearly two-thousand common English words, how many are triangle words?
*/

use regex::Regex;
use std::fs;

fn read_words() -> Vec<String> {
    return fs::read_to_string(euler::resources_path().join("p042_words.txt"))
        .unwrap()
        .split(",")
        .map(|name| {
            Regex::new("\"")
                .unwrap()
                .replace_all(name, "")
                .to_string()
                .to_lowercase()
        })
        .collect::<Vec<String>>();
}

fn is_triangle_number(number: usize) -> bool {
    let discrim_sqrt = ((8 * number + 1) as f64).sqrt();

    return discrim_sqrt.fract() == 0.0;
}

fn count_triangle_words(words: Vec<String>) -> usize {
    let mut count: usize = 0;

    for word in words {
        let mut sum = 0;

        for char in word.chars() {
            // Lowercase letters start in ASCII at 97
            sum += (char as usize) - 96;
        }

        if is_triangle_number(sum) {
            count += 1;
        }
    }

    return count;
}

pub fn main() {
    let words = read_words();
    let count = count_triangle_words(words);

    println!("Out of nearly 2000 common English words, {count} are triangle words!");
}
