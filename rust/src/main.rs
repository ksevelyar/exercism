mod diffie_hellman;

fn main() {
    dbg!(diffie_hellman::modular_exponentiation(5u64, 6u64, 23u64));
}
