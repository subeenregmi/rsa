mod maths;
mod rsa;

fn main() {
    let p = 23;
    let q = 17;

    let n = p * q;

    let mut acc = rsa::RSA::new();

    acc.set_private(n);

    acc.print();

    let c = acc.encrypt(String::from("hello world!\0"));

    println!("{:?}", c);

    let primes = maths::sieve_of_eratosthenes(1_000_000);

    println!("{}", primes.len());

    let choice = rand::random_range(0..primes.len());

    println!(
        "is prime -f : {}, is prime -mr : {}",
        maths::fermat_primality_test(primes[choice], 1000),
        maths::miller_rabin_test(primes[choice], 1000)
    )
}
