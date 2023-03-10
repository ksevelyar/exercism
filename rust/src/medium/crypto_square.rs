pub fn encrypt(input: &str) -> String {
    let chars: Vec<char> = input
        .to_lowercase()
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect();

    let count = chars.len() as i32;
    if count == 0 {
        return "".to_string();
    }

    let (c, _r) = (1..count)
        .flat_map(|c| (1..count).map(move |r| (c, r)))
        .find(|(c, r)| ((c - r) <= 1) && c >= r && c * r >= count)
        .unwrap();

    let rows = chars.chunks(c as usize).collect::<Vec<_>>();

    (0..c)
        .map(|c| {
            rows.iter()
                .map(move |r| r.get(c as usize).unwrap_or(&' '))
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[test]
fn test_example() {
    assert_eq!(
        encrypt("If man was meant to stay on the ground, god would have given us roots."),
        "imtgdvs fearwer mayoogo anouuio ntnnlvt wttddes aohghn  sseoau "
    )
}

#[test]
fn test_long() {
    assert_eq!(
        encrypt(
            r#"
We choose to go to the moon.

We choose to go to the moon in this decade and do the other things,

not because they are easy, but because they are hard, because that

goal will serve to organize and measure the best of our energies and

skills, because that challenge is one that we are willing to accept,

one we are unwilling to postpone, and one which we intend to win,

and the others, too.

-- John F. Kennedy, 12 September 1962
        "#
        ),
        (String::from("womdbudlmecsgwdwob enooetbsenaotioihe ")
            + "cwotcbeeaeunolnnnr henhaecrsrsealeaf1 ocieucavugetciwnk9 "
            + "ohnosauerithcnhde6 sotteusteehaegitn2 eohhtseotsatptchn  "
            + "tsiehetohatwtohee  oesrethrenceopwod  gtdtyhagbdhanoety  "
            + "ooehaetaesaresih1  tgcirygnsklewtne2  ooaneaoitilweptrs  "
            + "ttdgerazoleiaoese  hoesaeleflnlrnntp  etanshwaosgleedot  "
            + "mhnoyainubeiuatoe  oedtbrldreinnnojm "),
    )
}
