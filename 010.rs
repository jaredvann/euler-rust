/*

Project Euler Problem 10:

The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
Find the sum of all the primes below two million.

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
    let mut sum: u64 = 0;

    for i in 2..2_000_000 {
        if is_prime(i) {
            sum += i as u64;
        }
    }

    println!("{:?}", sum);

}