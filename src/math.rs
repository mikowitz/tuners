use num::{cast, one, zero, PrimInt};

pub(crate) fn two<T: PrimInt>() -> T {
    cast(2).unwrap()
}

pub(crate) fn reduce<T: PrimInt>(a: T, b: T) -> (T, T) {
    let g = gcd(a, b);
    (a / g, b / g)
}

fn gcd<T: PrimInt>(a: T, b: T) -> T {
    let mut a = a;
    let mut b = b;
    while a % b > zero() {
        let t = a % b;
        a = b;
        b = t;
    }
    b
}

pub(crate) fn normalize_pair<T: PrimInt>(a: T, b: T) -> (T, T) {
    let af: f32 = cast(a).unwrap();
    let bf: f32 = cast(b).unwrap();
    let f: f32 = af / bf;

    match f {
        f if f < 1. => normalize_pair(a * two(), b),
        f if f > 2. => normalize_pair(a, b * two()),
        f if (f - 2.).abs() < f32::EPSILON => (a, b),
        f if (f - 1.).abs() < f32::EPSILON => {
            if a == one() && b == one() {
                (one(), one())
            } else {
                (two(), one())
            }
        }
        _ => (a, b),
    }
}

pub(crate) fn greatest_prime_factor<T: PrimInt>(a: T) -> T {
    let mut a = a;
    let mut p = two();

    while a > one() {
        if a % p == zero() {
            a = a / p;
        } else {
            p = p + one();
        }
    }
    p
}
