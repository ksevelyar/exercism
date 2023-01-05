// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
#[derive(Debug)]
pub struct Error;

pub struct Scale<'a> {
    tonic: &'a str,
    intervals: &'a str,
}

const SHARPS: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

const FLATS: [&str; 12] = [
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];

impl<'a> Scale<'a> {
    pub fn new(tonic: &'a str, intervals: &'a str) -> Result<Scale<'a>, Error> {
        Ok(Scale { tonic, intervals })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Ok(Scale {
            tonic,
            intervals: "",
        })
    }

    pub fn enumerate(&self) -> Vec<String> {
        let notes = match self.tonic {
            "F" | "Bb" | "Eb" | "Ab" | "Db" | "Gb" => FLATS,
            _ => SHARPS,
        };

        let position = notes.iter().position(|x| *x == self.tonic);

        notes
            .iter()
            .cycle()
            .skip(position.unwrap())
            .take(13)
            .map(|x| x.to_string())
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
}
