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

mod options;


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

    let mut files_read = 0;

    let mut main_counts: BTreeMap<String, usize> = BTreeMap::new();

    // common groupings of words, and their misspellings
    let mut synonyms:Vec<(usize, Vec<&str>)> = vec![
        (0, vec!["house", "housing", "home", "rental", "rent", "condo", "apartment"]),
        (0, vec!["beautiful", "vibrant", "beauty"]),
        (0, vec!["marina", "water", "waterfront", "boardwalk", "launch", "boat", "beach", "beaches", "sea", "seaside", "ocean"]),
        (0, vec!["walk", "walking", "walks", "walkable", "sidewalk", "path", "paths", "trail", "trails", "pedestrian", "pedestrians"]),
        (0, vec!["bike", "bikes", "biking", "cycle", "cycling", "cycles"]),
        (0, vec!["green", "air", "natural", "nature", "river", "forest", "mountain", "mountains"]),
        (0, vec!["park", "parks"]),
        (0, vec!["parking", "traffic"]),
        (0, vec!["retail", "store", "stores", "shop", "shops", "shopping"]),
        (0, vec!["restaurant", "restaurants", "cafe", "cafes", "coffee"]),
        (0, vec!["senior", "seniors", "elder", "elderly", "retire", "retirement"]),
        (0, vec!["police", "law", "crime", "drug", "drugs"]),
        (0, vec!["medical", "med", "nurse", "dentist", "doctor", "dr", "health", "healthcare"]),
        (0, vec!["village", "small", "chill"]),
        (0, vec!["music", "art", "arts", "culture", "cultural"]),
        (0, vec!["tourist", "tourists", "tour", "tourism", "visitor", "visitors"]),
        (0, vec!["hotel", "hotels", "motel", "motels", "accommodation", "accommodations"]),
    ];

    if args.is_directory {
        for entry in glob("**/*.md").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    //println!("{:?}", path.display());
                    let words_from_file = find_words_in_each_line(path);
                    files_read += 1;

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
        files_read += 1;
    }

    // change from btree to a vec to sort by value and not by key
    let mut sorted_counts = Vec::from_iter(main_counts);
    sorted_counts.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    let mut how_many_found = 0;
    let mut sorted_ones_to_print: Vec<(String, usize)> = Vec::with_capacity(options::MAX_HOW_MANY_TO_PRINT as usize);
    let mut longest_word_length = 0;

    sorted_counts.retain_mut(|x| {
        x.1 += 1;
        for group in &mut synonyms {
            if group.1.contains(&x.0.as_str()) {
                group.0 += x.1;
                return false;
            }
            
        }
        return true; // keep the value here
    });

    synonyms.sort_by(|&(a, _), &(b, _)| b.cmp(&a));

    // find the ones to print
    for (key, value) in sorted_counts.iter() {
        // stop the loop so it doesn't fill the terminal
        if how_many_found > options::MAX_HOW_MANY_TO_PRINT {
            break;
        }

        // stop the loop when the count for each word is too low
        if value < &options::MIN_COUNT {
            break;
        }

        // skip to next iteration in the loop if the characters in a word are under the variable
        if key.len() < options::MIN_CHARACTER_COUNT {
            continue;
        }

        // skip to next iteration in the loop so it doesn't print out a word we aren't looking for
        if options::SKIP_WORDS.contains(&key.as_str()) {
            continue;
        }

        if longest_word_length < key.len() {
            longest_word_length = key.len();
        }

        how_many_found += 1;
        sorted_ones_to_print.push((key.clone(), *value));
    }

    println!("Individual words:");

    // print out all the ones found
    for (key, value) in sorted_ones_to_print.iter() {
        if options::ALIGN_TABS {
            // scuffed implementation assuming words aren't too long
            if key.len() < 8 {
                println!("{}\t\t{}", key, value);
            } else {
                println!("{}\t{}", key, value);
            }
        } else {
            println!("{}\t{}", key, value);
        }
    }

    println!("\n\nGrouped words:");

    for (value, strings) in synonyms {
        println!("{}\t{:?}", value, strings);
    }

    println!("Time taken to run: {:.2?}ms\tFiles read: {:?}\tWords found: {}\nLength of longest printed word: {}",
        the_time.elapsed().as_millis(), files_read, &sorted_counts.len(), longest_word_length);

    if atty::is(Stream::Stdout) {
        // ran from a terminal
    } else {
        // didn't run from a terminal, so pause the output so we can see what's on screen
        pause();
    }
    
}

fn find_words_in_each_line(path: std::path::PathBuf) -> BTreeMap<String, usize> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();

    let word_regex = Regex::new(r"[\w'\-\_]+").unwrap();

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

/// Pauses the terminal so the we can read the output before the terminal closes
fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to exit...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
