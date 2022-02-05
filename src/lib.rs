use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_reader(input: &str) -> BufReader<File> {
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);
    reader
}

pub fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();

        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

pub fn count_lines<T: BufRead + Sized>(reader: T) -> i32 {
    let mut line_count = 0;
    for _ in reader.lines() {
        line_count += 1;
    }

    line_count
}

pub fn count_words<T: BufRead + Sized>(reader: T) -> i32 {
    let mut word_count = 0;

    for line_ in reader.lines() {
        let line = line_.unwrap();
        for _ in line.split(' ') {
            word_count += 1;
        }
    }
    word_count
}

pub fn word_occurrences<T: BufRead + Sized>(reader: T, key: &str, ignore_case: bool) -> i32 {
    let mut word_occ = 0;
    for line_ in reader.lines() {
        if ignore_case {
            let line = line_.unwrap().to_lowercase();
            if line.contains(key.to_lowercase().as_str()) {
                word_occ += 1;
            }
        } else {
            let line = line_.unwrap();
            if line.contains(key) {
                word_occ += 1;
            }
        }
    }
    word_occ
}
