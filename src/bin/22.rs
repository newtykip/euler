/*
Problem 22 - Names scores

Using  names.txt  (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first names, begin by sorting it into alphabetical order. Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.
For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of 938 × 53 = 49714.
What is the total of all the name scores in the file?
*/

use euler::RESOURCES;
use regex::Regex;
use std::fs;

fn read_names() -> Vec<String> {
    let mut names = fs::read_to_string(RESOURCES.join("p022_names.txt"))
        .unwrap()
        .split(",")
        .map(|name| Regex::new("\"").unwrap().replace_all(name, "").to_string())
        .collect::<Vec<String>>();

    names.sort();

    return names;
}

fn name_score(name: String, position: usize) -> usize {
    let mut letter_sum: usize = 0;

    for char in name.chars() {
        // Capital letters start in ASCII at 65
        letter_sum += (char as usize) - 64;
    }

    return letter_sum * position;
}

fn name_score_total(names: Vec<String>) -> usize {
    let mut total: usize = 0;

    for i in 0..names.len() {
        total += name_score(names[i].clone(), i + 1);
    }

    return total;
}

pub fn main() {
    let names = read_names();
    let total = name_score_total(names);

    println!("The total of all of the name scores in the file is {total}");
}
