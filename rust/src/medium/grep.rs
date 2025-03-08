use anyhow::Error;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Default)]
pub struct Flags {
    line_numbers: bool,
    only_names: bool,
    case_insensitive: bool,
    invert: bool,
    entire_lines: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        flags
            .iter()
            .fold(Flags::default(), |mut flags, x| match *x {
                "-n" => {
                    flags.line_numbers = true;
                    flags
                }
                "-l" => {
                    flags.only_names = true;
                    flags
                }
                "-i" => {
                    flags.case_insensitive = true;
                    flags
                }
                "-v" => {
                    flags.invert = true;
                    flags
                }
                "-x" => {
                    flags.entire_lines = true;
                    flags
                }
                _ => flags,
            })
    }
}

fn is_match(line: &str, pattern: &str, flags: &Flags) -> bool {
    let is_match = match flags {
        Flags {
            entire_lines: true,
            case_insensitive: true,
            ..
        } => line.to_lowercase() == (pattern.to_lowercase()),
        Flags {
            entire_lines: true,
            case_insensitive: false,
            ..
        } => line == pattern,
        Flags {
            entire_lines: false,
            case_insensitive: true,
            ..
        } => line.to_lowercase().contains(&pattern.to_lowercase()),
        Flags {
            entire_lines: false,
            case_insensitive: false,
            ..
        } => line.contains(pattern),
    };

    if flags.invert { !is_match } else { is_match }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let sources: Result<Vec<_>, Error> = files
        .iter()
        .map(|file_name| {
            let file = File::open(file_name)?;
            Ok(BufReader::new(file)
                .lines()
                .flatten()
                .enumerate()
                .filter(|(_, line)| is_match(line, pattern, flags))
                .map(|(ind, line)| {
                    let line = if flags.line_numbers {
                        format!("{}:{}", ind + 1, line)
                    } else {
                        line
                    };

                    if flags.only_names {
                        return file_name.to_string();
                    };

                    if files.len() > 1 {
                        format!("{}:{}", file_name, line)
                    } else {
                        line
                    }
                })
                .collect::<Vec<String>>())
        })
        .filter(|d| match d.as_deref() {
            Ok([]) => false,
            _ => true,
        })
        .collect();

    let mut sources: Vec<_> = sources?.into_iter().flatten().collect();
    sources.dedup();

    Ok(sources.into_iter().collect())
}
