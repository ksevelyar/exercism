use rand::Rng;

fn is_odd(num: u64) -> bool {
    num & 1 == 1
}

fn modular_multiply(a: u64, b: u64, modulus: u64) -> u64 {
    (a * b) % modulus
}

fn modular_exponentiation_with_acc(acc: u64, base: u64, exponent: u64, modulus: u64) -> u64 {
    if exponent == 0 {
        return acc;
    }

    let updated_acc = match is_odd(exponent) {
        true => modular_multiply(acc, base, modulus),
        false => acc,
    };
    let exponent_divided_by_two = exponent >> 1;
    let updated_base = modular_multiply(base, base, modulus);

    modular_exponentiation_with_acc(updated_acc, updated_base, exponent_divided_by_two, modulus)
}

fn modular_exponentiation(base: u64, exponent: u64, modulus: u64) -> u64 {
    let base = base % modulus;
    modular_exponentiation_with_acc(1, base, exponent, modulus)
}

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponentiation(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponentiation(b_pub, a, p)
}
