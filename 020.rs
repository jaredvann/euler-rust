/*

Project Euler Problem 20:

n! means n × (n − 1) × ... × 3 × 2 × 1

For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

Find the sum of the digits in the number 100!

*/


fn main() {
    let x = 100;

    let mut r = vec![0; 200];

    r[0] = 1;

    for i in 1..x+1 {
        let mut carry = 0;

        for j in 0..200 {
            r[j] = r[j] * i + carry;
            carry = 0;

            while r[j] > 9 {
                r[j] -= 10;
                carry += 1;
            }
        }
    }

    println!("{:?}", r.iter().fold(0, |a, &b| a + b));
}