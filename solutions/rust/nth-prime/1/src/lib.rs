pub fn nth(mut n: u32) -> u32 {
        fn is_prime(number: u32) -> bool {
        if number < 2 {
            return false;
        }

        for i in 2..=number.isqrt() {
            if number % i == 0 {
                return false;
            }
        }
        return true;
    }
    let mut prime_number = 2;

    loop {
        if n == 0 {
            return prime_number;
        }

        prime_number += 1;
        if is_prime(prime_number) {
            n -= 1;
        }
    }

}
