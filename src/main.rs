use std::collections::BTreeMap;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::iter::FromIterator;

use regex::Regex;

fn main() {
    let min_count = 10;
    let skip_words = ["to", "the", "a", "of", "in", "not", "with", "and",
        "for", "on", "is", "be", "or", "at", "as", "from", "that", "are", "it", "by",
        "all", "up", "like", "i", "just", "our", "use", "no", "an", "but", "we", "there",
        "too", "do", "have"];

    let word_regex = Regex::new(r"[\w']+").unwrap();

    let mut counts: BTreeMap<String, isize> = BTreeMap::new();

    if let Ok(lines) = read_lines("./haystack.txt") {
        for line in lines.flatten() {
            let line = line.to_lowercase();
            let matches = word_regex.find_iter(&line);

            for word in matches.into_iter() {
                let word = word.as_str();
                *counts.entry(word.into()).or_insert(0) += 1;
            }
        }
    }

    // change from btree to a vec to sort by value and not by key
    let mut sorted_counts = Vec::from_iter(counts);
    sorted_counts.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    // print out the newly sorted words and their counts
    for (key, value) in sorted_counts.iter() {

        // stop the loop when the count for each word is too low
        if value < &min_count {
            break;
        }

        // skip to next iteration in the loop so it doesn't print out a word we aren't looking for
        if skip_words.contains(&key.as_str()) {
            continue;
        }

        println!("{} {}", key, value);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
