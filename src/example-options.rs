// max of how many words to print out to console
const MAX_HOW_MANY_TO_PRINT: isize = 60;

// minimum number of characters a word needs to be printed out
const MIN_CHARACTER_COUNT: usize = 3;

// only show words that have been mentioned at least this number of times
const MIN_COUNT: isize = 3;

// skip printing out these words and their times mentioned
const SKIP_WORDS: [&str; 36] = ["to", "the", "a", "of", "in", "not", "with", "and",
    "for", "on", "is", "be", "or", "at", "as", "from", "that", "are", "it", "by",
    "all", "up", "like", "i", "just", "our", "use", "no", "an", "but", "we", "there",
    "too", "do", "have", "they"];

pub const ALIGN_TABS: bool = true;
