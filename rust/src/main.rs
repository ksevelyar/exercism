mod medium;

fn main() {
    dbg!(medium::anagram::anagrams_for(
        "listen",
        &["enlists", "google", "inlets", "banana"]
    ));
}
