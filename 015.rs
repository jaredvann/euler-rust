/*

Project Euler Problem 15:

Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
https://projecteuler.net/project/images/p015.gif

How many such routes are there through a 20×20 grid?

*/


fn main() {
    let s = 20;
    let mut n: i64 = 1;

    for i in 0..20 {
        n *= 2*s - i;
        n /= i + 1;
    }

    println!("{:?}", n);
}

// I cheated. :(