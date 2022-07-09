pub fn raindrops(n: u32) -> String {
    let factors = vec![3, 5, 7].into_iter().filter(|factor| n % factor == 0);

    let result = factors
        .map(|factor| match factor {
            3 => "Pling",
            5 => "Plang",
            7 => "Plong",
            _ => "",
        })
        .collect::<Vec<&str>>()
        .join("");

    if result.is_empty() {
        return n.to_string();
    }
    result
}
