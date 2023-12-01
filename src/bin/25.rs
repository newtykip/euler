/*
Problem 25 - $1000$-digit Fibonacci Number

The Fibonacci sequence is defined by the recurrence relation:
$F_n = F_{n - 1} + F_{n - 2}$, where $F_1 = 1$ and $F_2 = 1$.
Hence the first $12$ terms will be:
\begin{align}
F_1 &= 1\\
F_2 &= 1\\
F_3 &= 2\\
F_4 &= 3\\
F_5 &= 5\\
F_6 &= 8\\
F_7 &= 13\\
F_8 &= 21\\
F_9 &= 34\\
F_{10} &= 55\\
F_{11} &= 89\\
F_{12} &= 144
\end{align}
The $12$th term, $F_{12}$, is the first term to contain three digits.
What is the index of the first term in the Fibonacci sequence to contain $1000$ digits?
*/

fn fibonacci_with_digits(digits: usize) -> usize {
    let mut antepenultimate = vec![1];
    let mut previous = vec![1];
    let mut index = 2;

    while previous.len() < digits {
        let mut current = vec![0; previous.len().max(antepenultimate.len())];
        let mut carry = 0;

        for i in 0..current.len() {
            let sum = if i < antepenultimate.len() {
                antepenultimate[i]
            } else {
                0
            } + if i < previous.len() { previous[i] } else { 0 }
                + carry;
            current[i] = sum % 10;
            carry = sum / 10;
        }

        while carry > 0 {
            current.push(carry % 10);
            carry /= 10;
        }

        antepenultimate = previous;
        previous = current;
        index += 1;
    }

    index
}

pub fn main() {
    let index: usize = fibonacci_with_digits(1000);
    println!(
        "The index of the first term in the Fibonacci sequence to contain 1000 digits is {}!",
        index
    );
}
