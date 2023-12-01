/*
Problem 24 - Lexicographic Permutations

A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically, we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:
012   021   102   120   201   210
What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
*/

fn permutations<T>(array: Vec<T>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut out = vec![];

    match array.len() {
        0 => vec![],
        1 => vec![array],
        _ => {
            for i in 0..array.len() {
                let mut array = array.clone();
                let first = array.remove(i);
                let mut permutations = permutations(array);

                for permutation in permutations.iter_mut() {
                    permutation.insert(0, first.clone());
                }

                out.append(&mut permutations);
            }

            out
        }
    }
}

fn nth_lexicographic_permutation(array: Vec<usize>, n: usize) -> Vec<usize> {
    let mut permutations = permutations(array);
    permutations.sort();

    permutations[n - 1].clone()
}

pub fn main() {
    let digits = (0..10).collect::<Vec<usize>>();
    let millionth_permutation = nth_lexicographic_permutation(digits, 1_000_000);

    println!(
        "The millionth lexicographic permutation of the digits 0-9 is {}!",
        millionth_permutation
            .iter()
            .map(|&d| d.to_string())
            .collect::<String>()
    );
}
