/*
Problem 67 - Maximum path sum II

By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top to bottom is 23.
3     7   4
2   4   6
8 5   9   3
That is, 3 + 7 + 4 + 9 = 23.
Find the maximum total from top to bottom in  triangle.txt  (right click and 'Save Link/Target As...'), a 15K text file containing a triangle with one-hundred rows.
NOTE:  This is a much more difficult version of  Problem 18 . It is not possible to try every route to solve this problem, as there are 2 99  altogether! If you could check one trillion (10 12 ) routes every second it would take over twenty billion years to check them all. There is an efficient algorithm to solve it. ;o)
*/

use euler::RESOURCES;
use std::fs;

fn read_triangle() -> Vec<Vec<usize>> {
    return fs::read_to_string(RESOURCES.join("p067_triangle.txt"))
        .unwrap()
        .split("\n")
        .map(|name| {
            name.split(' ')
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
}

fn maximum_path_sum(triangle: Vec<Vec<usize>>) -> usize {
    let mut tri = triangle.clone();

    for row in (0..(triangle.len() - 1)).rev() {
        for col in 0..(row + 1) {
            tri[row][col] = tri[row][col] + std::cmp::max(tri[row + 1][col], tri[row + 1][col + 1]);
        }
    }

    return tri[0][0];
}

pub fn main() {
    let triangle = read_triangle();
    let sum = maximum_path_sum(triangle);
    println!("The maximum path sum of the triangle is {}!", sum);
}
