/// Taken from https://github.com/kunerd/rpaillier/blob/master/src/bigint_extensions.rs

use num::traits::{ Zero, One, FromPrimitive, ToPrimitive };
use num::bigint::{ BigInt, Sign };

pub trait ModPow<T> {
    fn mod_pow(&self, exp: &T, m: &T) -> T;
}

const TABLE_BASE: usize = 5;

impl ModPow<BigInt> for BigInt {

    // Left-to-right k-ary exponentiation
    fn mod_pow(&self, exp: &BigInt, m: &BigInt) -> BigInt {

        let base = 2 << (TABLE_BASE - 1);

        let mut table = Vec::with_capacity(base);
        table.push(BigInt::one());

        for i in 1..base {
            let last = table.get_mut(i-1).unwrap().clone();

            table.push((last * self) % m);
        }

        let mut r = BigInt::one();

        for i in digits_of_n(exp, base).iter().rev() {
            for _ in 0..TABLE_BASE {
                r = &r * &r % m
            }

            if *i != 0 {
                r = &r * table.get(*i).unwrap() % m;
            }
        }

        r
    }
}

fn digits_of_n(e: &BigInt, b: usize) -> Vec<usize> {
    let mut digits = Vec::new();

    let mut n = (*e).clone();
    let base = BigInt::from_usize(b).unwrap();

    while n > BigInt::zero() {
        digits.push((&n % &base).to_usize().unwrap());
        n = &n / &base;
    }

    digits
}
