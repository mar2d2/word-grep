use clap::{App, Arg};
use regex::Regex;
use std::io;

use grep_lite::{count_lines, count_words, get_reader, process_lines, word_occurrences};

fn main() {
    let matches = App::new("grep-lite")
        .version("0.1")
        .about(
            "Searches for patterns in a file.\nGet count of words or lines in a file.\nGet occurrences of a specific word in a file",
        )
        .arg(
            Arg::new("pattern")
                .long("pattern")
                .short('p')
                .value_name("PATTERN")
                .help("The pattern to search for")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new("count_lines")
                .long("count-lines")
                .help("Counts lines in file")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::new("count_words")
                .long("count-words")
                .help("Counts the words in file")
                .takes_value(false),
        )
        .arg(
            Arg::new("word_occurrences")
                .long("word-occurrences")
                .help("Counts the occurrences of a given word in file")
                .takes_value(true)
                .value_name("WORD")
                .required(false),
        )
        .arg(
            Arg::new("ignore_case")
                .long("ignore-case")
                .short('i')
                .help("ignore caseing in word occurrence")
                .takes_value(false),
        )
        .arg(
            Arg::new("input")
                .help("File to search")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    if let Some(i) = matches.value_of("pattern") {
        let pattern = i;
        let input = matches.value_of("input").unwrap_or("-");
        let re = Regex::new(pattern).unwrap();

        if input == "-" {
            let stdin = io::stdin();
            let reader = stdin.lock();

            process_lines(reader, re);
        } else {
            let reader = get_reader(input);
            process_lines(reader, re);
        }
    } else if matches.is_present("count_lines") {
        let input = matches.value_of("input").unwrap();
        let reader = get_reader(input);
        println!("There are {} lines in file: {}", count_lines(reader), input);
    } else if matches.is_present("count_words") {
        let input = matches.value_of("input").unwrap();
        let reader = get_reader(input);
        println!("There are {} words in file: {}", count_words(reader), input);
    } else if let Some(i) = matches.value_of("word_occurrences") {
        let key = i;
        let input = matches.value_of("input").unwrap();
        let ignore_case = matches.is_present("ignore_case");
        let reader = get_reader(input);
        println!(
            "There are {} occurrences of the word {} in file {}",
            word_occurrences(reader, key, ignore_case),
            key,
            input
        );
    }
}
