/*

Project Euler Problem 4:

A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
Find the largest palindrome made from the product of two 3-digit numbers.

*/


fn main() {
    let mut l = 0;

    for x in 0..1000 {
        for y in 0..1000 {
            let z = x * y;

            let s: String = z.to_string();
            let s_rev: String = s.chars().rev().collect();

            if z > l && s == s_rev {
                l = z;
            }
        }
    }

    println!("{:?}", l);
}