/*

Project Euler Problem 14:

The following iterative sequence is defined for the set of positive integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:

13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1

It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?
Note: Once the chain starts the terms are allowed to go above one million.

*/


fn main() {
    let mut l1: i64 = 0;
    let mut l2: i64 = 0;

    for i in 1..1_000_000 {
        let mut n = i;
        let mut s = 0;

        while n != 1 {
            if n % 2 == 0 {
                n = n/2;
            } else {
                n = 3*n + 1;
            }

            s += 1;
        }

        if s > l1 {
            l1 = s;
            l2 = i;
        }
    }

    println!("{:?}", l2);
}