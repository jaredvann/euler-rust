/*

Project Euler Problem 16:

2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
What is the sum of the digits of the number 2^1000?

*/


fn main() {
    let mut v = vec![0; 1000];

    v[0] = 2;

    for _ in 1..1000 {
        let mut c = 0;

        for j in 0..1000 {
            v[j] = v[j]*2 + c;
            c = 0;

            while v[j] > 9 {
                v[j] -= 10;
                c += 1;
            }
        }
    }

    let s: i32 = v.iter().sum();

    println!("{:}", s);
}
