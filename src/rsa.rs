use crate::maths::{self, mod_exp};

use rand::Rng;

pub struct RSA {
    // d
    private_key: Option<u32>,

    // (n, e)
    pub public_key: Option<(u32, u32)>,
}

impl RSA {
    pub fn new() -> RSA {
        RSA {
            private_key: None,
            public_key: None,
        }
    }

    pub fn set_private(&mut self, n: u32) -> &RSA {
        let lambda = maths::carmicheal(n);

        let mut rng = rand::rng();

        let e = loop {
            let e: u32 = rng.random_range(1..lambda);
            if maths::coprime(e, lambda) {
                break e;
            }
        };

        let d = maths::mod_inverse(e, lambda);

        self.private_key = Some(d);
        self.public_key = Some((n, e));

        self
    }

    pub fn print(&self) {
        println!(
            "public key: {:?}, private key: {:?}",
            self.public_key, self.private_key
        );
    }

    pub fn encrypt(&self, message: String) -> Vec<u32> {
        let bytes = message.into_bytes();
        let mut code = Vec::new();

        for chunk in bytes.chunks(4) {
            let m: u32;

            if chunk.len() != 4 {
                let mut temp_chunk: [u8; 4] = [0, 0, 0, 0];
                for i in 0..chunk.len() {
                    temp_chunk[chunk.len() - 1 - i] = chunk[chunk.len() - 1 - i];
                }
                m = u32::from_ne_bytes(temp_chunk.try_into().unwrap());
            } else {
                m = u32::from_ne_bytes(chunk.try_into().unwrap());
            }

            let c = mod_exp(m, self.public_key.unwrap().1, self.public_key.unwrap().0);

            code.push(c);
        }

        return code;
    }
}
