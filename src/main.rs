use std::collections::HashSet;

use rand::Rng;

fn factors(n: u32) -> HashSet<u32> {
    let mut factors = HashSet::new();

    for i in 1..=n {
        if n % i == 0 {
            factors.insert(i);
        }
    }

    return factors;
}

fn mod_exp(a: u32, b: u32, n: u32) -> u32 {
    let mut temp = 1;

    for _ in 0..b {
        temp = (temp * a) % n;
    }

    return temp;
}

fn coprime(a: u32, b: u32) -> bool {
    let a_factors = factors(a);
    let b_factors = factors(b);

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

fn carmicheal(n: u32) -> u32 {
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

fn mod_inverse(a: u32, n: u32) -> u32 {
    let mut x = 1;
    loop {
        if (x * a) % n == 1 {
            return x;
        }

        x += 1;
    }
}

fn main() {
    let p = 23;
    let q = 17;

    let n = p * q;

    let lambda = carmicheal(n);

    println!("{}", lambda);

    let mut rng = rand::rng();

    let e = loop {
        let e: u32 = rng.random_range(1..lambda);
        if coprime(e, lambda) {
            break e;
        }
    };

    let d = mod_inverse(e, lambda);

    let public_key = (n, e);
    let private_key = d;

    println!("public key: {:?}, private key: {}", public_key, private_key);
}
