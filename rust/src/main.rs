mod diffie_hellman;

fn main() {
    dbg!(diffie_hellman::secret(4_294_967_927, 843, 4_294_967_300));
}
