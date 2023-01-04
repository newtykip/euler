/*
Problem 14 - Longest Collatz sequence

The following iterative sequence is defined for the set of positive integers:
n  →  n /2 ( n  is even)  n  → 3 n  + 1 ( n  is odd)
Using the rule above and starting with 13, we generate the following sequence:
13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
Which starting number, under one million, produces the longest chain?
NOTE:  Once the chain starts the terms are allowed to go above one million.
*/

use std::collections::HashMap;

fn f(n: usize) -> usize {
    if n % 2 == 0 {
        return n / 2;
    } else {
        return ((3 * n) + 1) / 2;
    }
}

fn longest_collatz_sequence(greatest_start: usize) -> usize {
    let mut sequence_lengths: HashMap<usize, usize> = HashMap::new();

    for i in 1..(greatest_start + 1) {
        let mut sequence_length: usize = 0;
        let mut current_number = i;

        while current_number > 1 {
            current_number = f(current_number);

            // Eventually the sequence will break down into one we have already computed the length for!
            if sequence_lengths.contains_key(&current_number) {
                sequence_length += *sequence_lengths.get(&current_number).unwrap();
                break;
            }

            sequence_length += 1;
        }

        sequence_lengths.insert(i, sequence_length);
    }

    return *sequence_lengths.iter().max_by_key(|x| x.1).unwrap().0;
}

fn main() {
    let starting_number = longest_collatz_sequence(1_000_000);

    println!(
        "The starting number which produces the longest collatz sequence is {starting_number}"
    );
}
