/*

Project Euler Problem 3:

The prime factors of 13195 are 5, 7, 13 and 29.
What is the largest prime factor of the number 600851475143 ?

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
    let x: i64 = 600851475143;

    for i in (0..((x as f64).sqrt() as i64)).rev() {
        if x % i == 0 && is_prime(i) {
            println!("{:?}", i);
            return;
        }
    }
}