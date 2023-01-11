#[derive(Debug)]
pub struct Error;

pub struct Scale<'a> {
    tonic: &'a str,
    intervals: Vec<char>,
}

const SHARPS: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

const FLATS: [&str; 12] = [
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];

impl<'a> Scale<'a> {
    pub fn new(tonic: &'a str, intervals: &'a str) -> Result<Scale<'a>, Error> {
        Ok(Scale {
            tonic,
            intervals: intervals.chars().collect(),
        })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Ok(Scale {
            tonic,
            intervals: "mmmmmmmmmmmm".chars().collect(),
        })
    }

    pub fn enumerate(&self) -> Vec<String> {
        let notes = match self.tonic {
            "C" | "a" => SHARPS,
            "G" | "D" | "A" | "E" | "B" | "F#" => SHARPS,
            "e" | "b" | "f#" | "c#" | "g#" | "d#" => SHARPS,
            _ => FLATS,
        };

        let start = notes
            .iter()
            .position(|x| *x.to_lowercase() == self.tonic.to_lowercase())
            .unwrap();

        let intervals = self
            .intervals
            .iter()
            .fold(vec![start], |mut acc, interval| {
                let last = acc.last().unwrap();

                let shift = last
                    + match interval {
                        'm' => 1,
                        'M' => 2,
                        'A' => 3,
                        _ => panic!(),
                    };

                acc.push(shift);
                acc
            });

        notes
            .iter()
            .cycle()
            .take(*intervals.last().unwrap() + 1)
            .enumerate()
            .filter(|(ind, _x)| intervals.contains(ind))
            .map(|(_ind, x)| x.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn process_chromatic_case(tonic: &str, expected: &[&str]) {
        let s = Scale::chromatic(tonic).unwrap();

        assert_eq!(s.enumerate(), expected);
    }

    fn process_interval_case(tonic: &str, intervals: &str, expected: &[&str]) {
        let s = Scale::new(tonic, intervals).unwrap();

        assert_eq!(s.enumerate(), expected);
    }

    #[test]
    fn test_chromatic_scale_with_sharps() {
        process_chromatic_case(
            "C",
            &[
                "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B", "C",
            ],
        );
    }

    #[test]
    fn test_chromatic_scale_with_flats() {
        process_chromatic_case(
            "F",
            &[
                "F", "Gb", "G", "Ab", "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F",
            ],
        );
    }

    #[test]
    fn test_simple_major_scale() {
        process_interval_case("C", "MMmMMMm", &["C", "D", "E", "F", "G", "A", "B", "C"]);
    }

    #[test]
    fn test_major_scale_with_sharps() {
        process_interval_case("G", "MMmMMMm", &["G", "A", "B", "C", "D", "E", "F#", "G"]);
    }

    #[test]
    fn test_dorian_mode() {
        process_interval_case("d", "MmMMMmM", &["D", "E", "F", "G", "A", "B", "C", "D"]);
    }

    #[test]
    fn test_hexatonic() {
        process_interval_case("Db", "MMMMMM", &["Db", "Eb", "F", "G", "A", "B", "Db"]);
    }

    #[test]
    fn test_locrian_mode() {
        process_interval_case(
            "g",
            "mMMmMMM",
            &["G", "Ab", "Bb", "C", "Db", "Eb", "F", "G"],
        );
    }
}
