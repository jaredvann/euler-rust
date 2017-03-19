/*

Project Euler Problem 7:

By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
What is the 10 001st prime number?

*/


fn is_prime(x: i64) -> bool {
    if x == 2 || x == 3 {
        return true;
    } else if x % 2 == 0 || x % 3 == 0 {
        return false;
    }

    let (mut i, mut w) = (5i64, 2i64);

    while i * i <= x {
        if x % i == 0 {
            return false;
        }
        i += w;
        w = 6 - w;
    }
    
    return true;
}

fn main() {
    let mut n = 0;

    for i in 1.. {
        if is_prime(i) {
            n += 1;

            if n == 10001 {
                println!("{:?}", i);
                return;
            }
        }
    }
}