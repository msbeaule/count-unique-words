use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::iter::FromIterator;
use std::time::Instant;
use std::io::{stdin, stdout, Read, Write};

use regex::Regex;
use clap::Parser;
use glob::glob;
use atty::Stream;

/// Pauses the terminal so the we can read the output before the terminal closes
fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to exit...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

#[derive(Parser)]
struct Cli {
    /// The path to the .txt file
    path: std::path::PathBuf,

    #[arg(short = 'd')]
    is_directory: bool,
}

fn main() {
    let the_time = Instant::now();

    let args = Cli::parse();

    let max_how_many_to_print = 40;

    // only show words that have been mentioned at least this number of times
    let min_count = 10;

    // skip printing out these words and their times mentioned
    let skip_words = ["to", "the", "a", "of", "in", "not", "with", "and",
        "for", "on", "is", "be", "or", "at", "as", "from", "that", "are", "it", "by",
        "all", "up", "like", "i", "just", "our", "use", "no", "an", "but", "we", "there",
        "too", "do", "have"];

    let mut main_counts: BTreeMap<String, isize> = BTreeMap::new();

    if args.is_directory {
        for entry in glob("**/*.md").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    println!("{:?}", path.display());
                    let words_from_file = find_words_in_each_line(path);

                    for word_and_count in words_from_file {
                        let word = word_and_count.0;
                        let count = word_and_count.1;

                        // adds the word count from the file to the main word count variable
                        *main_counts.entry(word.into()).or_insert(count) += count;
                    }
                    
                }
                Err(e) => println!("{:?}", e),
            }
        }
    } else {
        main_counts = find_words_in_each_line(args.path);
    }

    // change from btree to a vec to sort by value and not by key
    let mut sorted_counts = Vec::from_iter(main_counts);
    sorted_counts.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    let mut how_many_printed = 0;

    // print out the newly sorted words and their counts
    for (key, value) in sorted_counts.iter() {
        // stop the loop so it doesn't fill the terminal
        if how_many_printed > max_how_many_to_print {
            break;
        }

        // stop the loop when the count for each word is too low
        if value < &min_count {
            break;
        }

        // skip to next iteration in the loop so it doesn't print out a word we aren't looking for
        if skip_words.contains(&key.as_str()) {
            continue;
        }

        how_many_printed += 1;
        println!("{} {}", key, value);
    }

    println!("Time taken to run: {:.2?}", the_time.elapsed());

    if atty::is(Stream::Stdout) {
        // ran from a terminal
    } else {
        // didn't run from a terminal, so pause the output so we can see what's on screen
        pause();
    }
    
}

fn find_words_in_each_line(path: std::path::PathBuf) -> BTreeMap<String, isize> {
    let mut counts: BTreeMap<String, isize> = BTreeMap::new();

    let word_regex = Regex::new(r"[\w']+").unwrap();

    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            let line = line.to_lowercase();
            let matches = word_regex.find_iter(&line);

            for word in matches.into_iter() {
                let word = word.as_str();
                *counts.entry(word.into()).or_insert(0) += 1;
            }
        }
    }

    return counts;

    
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
