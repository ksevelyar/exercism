#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    };

    let aliquot_sum: u64 = (1..num).filter(|factor| num % factor == 0).sum();

    match (aliquot_sum, num) {
        (a, b) if a == b => Some(Classification::Perfect),
        (a, b) if a > b => Some(Classification::Abundant),
        (a, b) if a < b => Some(Classification::Deficient),
        (_, _) => None,
    }
}
