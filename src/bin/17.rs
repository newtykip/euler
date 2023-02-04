/*
Problem 17 - Number letter counts

If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
NOTE:  Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.
*/

use std::ops::Div;

use phf::phf_map;

const TRANSLATIONS: phf::Map<u16, &'static str> = phf_map! {
    0u16 => "",
    1u16 => "one",
    2u16 => "two",
    3u16 => "three",
    4u16 => "four",
    5u16 => "five",
    6u16 => "six",
    7u16 => "seven",
    8u16 => "eight",
    9u16 => "nine",
    10u16 => "ten",
    11u16 => "eleven",
    12u16 => "twelve",
    13u16 => "thirteen",
    14u16 => "fourteen",
    15u16 => "fifteen",
    16u16 => "sixteen",
    17u16 => "seventeen",
    18u16 => "eighteen",
    19u16 => "nineteen",
    20u16 => "twenty",
    30u16 => "thirty",
    40u16 => "forty",
    50u16 => "fifty",
    60u16 => "sixty",
    70u16 => "seventy",
    80u16 => "eighty",
    90u16 => "ninety"
};

fn number_to_words(number: usize) -> String {
    let mut n = number.clone();
    let mut out = String::new();

    // Thousands
    let thousands = n.div(1000);
    n -= thousands * 1000;

    if thousands > 0 {
        out = format!("{}thousand", TRANSLATIONS.get(&(thousands as u16)).unwrap());
    }

    // Hundreds
    let hundreds = n.div(100);
    n -= hundreds * 100;

    if hundreds > 0 {
        out = format!("{out}{}hundred", TRANSLATIONS.get(&(hundreds as u16)).unwrap());
    }

    // Tens
    let tens = n.div(10);
    n -= tens * 10;

    if tens > 0 {
        let translation: &str;
        let and = if hundreds > 0 {"and"} else {""};

        if n % 10 > 0 && tens == 1 {
            translation = TRANSLATIONS.get(&((10 + (n % 10)) as u16)).unwrap();
            
            n -= n % 10;
        } else {
            translation = TRANSLATIONS.get(&(tens as u16 * 10)).unwrap();
        }

        out = format!("{out}{and}{}", translation);
    }

    // Remainder
    if n > 0 {
        let and = if hundreds > 0 && tens <= 0 {"and"} else {""};
        out = format!("{out}{and}{}", TRANSLATIONS.get(&(n as u16)).unwrap());
    }

    out
}

fn letter_count(upper_bound: usize) -> usize {
    let mut sum = 0;

    for i in 1..(upper_bound + 1) {
        println!("{} {}", number_to_words(i), number_to_words(i).len());

        sum += number_to_words(i).len();
    }

    sum
}

fn main() {
    let sum = letter_count(1000);

    println!("{} letters would be used to write out all of the numbers from 1 to 1000!", sum);
}
