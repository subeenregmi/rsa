use std::collections::HashSet;

pub fn factor(n: u32) -> HashSet<u32> {
    let mut factors = HashSet::new();

    for i in 1..=n {
        if n % i == 0 {
            factors.insert(i);
        }
    }

    return factors;
}

pub fn mod_exp(a: u32, b: u32, n: u32) -> u32 {
    let mut temp: u64 = 1;

    for _ in 0..b {
        temp = (temp * a as u64) % n as u64;
    }

    return u32::try_from(temp).unwrap();
}

pub fn coprime(a: u32, b: u32) -> bool {
    let a_factors = factor(a);
    let b_factors = factor(b);

    for factor in a_factors {
        if factor == 1 {
            continue;
        }

        if b_factors.contains(&factor) {
            return false;
        }
    }

    return true;
}

pub fn carmicheal(n: u32) -> u32 {
    let mut comprimes: Vec<u32> = Vec::new();

    for a in 1..n {
        if coprime(a, n) {
            comprimes.push(a);
        }
    }

    let mut m: u32 = 1;

    loop {
        let mut found = true;

        for a in comprimes.iter() {
            if (mod_exp(*a, m, n)) != 1 {
                found = false;
                break;
            }
        }

        if !found {
            m += 1;
            continue;
        }

        break;
    }

    return m;
}

pub fn mod_inverse(a: u32, n: u32) -> u32 {
    let mut x = 1;
    loop {
        if (x * a) % n == 1 {
            return x;
        }

        x += 1;
    }
}

// Find all primes upto n
pub fn sieve_of_eratosthenes(n: u32) -> Vec<u32> {
    let numbers = Vec::from_iter(2..=n);
    let mut is_prime = vec![true; numbers.len()];

    for i in 2..=n.isqrt() {
        //println!("{i}, {}, {}", i * i - 2, numbers[(i * i - 2) as usize]);
        if !is_prime[(i - 2) as usize] {
            continue;
        }

        for j in ((i * i - 2)..numbers.len() as u32).step_by(i as usize) {
            is_prime[j as usize] = false;
        }
    }

    let mut primes: Vec<u32> = Vec::new();
    for i in 0..numbers.len() {
        if is_prime[i] {
            primes.push(numbers[i]);
        }
    }

    primes
}
