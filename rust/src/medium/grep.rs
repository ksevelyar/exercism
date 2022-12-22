use anyhow::Error;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug)]
pub struct Flags<'a> {
    flags: &'a [&'a str],
}

impl<'a> Flags<'a> {
    pub fn new(flags: &'a [&'a str]) -> Self {
        Self { flags }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let sources: Result<Vec<_>, Error> = files
        .iter()
        .map(|file| {
            let file = File::open(file)?;
            Ok(BufReader::new(file)
                .lines()
                .flatten()
                .filter(|line| line.contains(pattern))
                .collect::<Vec<String>>())
        })
        .filter(|d| match d.as_deref() {
            Ok([]) => false,
            _ => true,
        })
        .collect();

    match sources {
        Ok(vec) => Ok(vec.into_iter().flatten().collect()),
        Err(err) => Err(err),
    }
}
