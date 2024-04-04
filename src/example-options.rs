// max of how many words to print out to console
pub const MAX_HOW_MANY_TO_PRINT: usize = 60;

// minimum number of characters a word needs to be printed out
pub const MIN_CHARACTER_COUNT: usize = 3;

// only show words that have been mentioned at least this number of times
pub const MIN_COUNT: usize = 3;

// skip printing out these words and their times mentioned
pub const SKIP_WORDS: [&str; 36] =
    ["to", "the", "a", "of", "in", "not", "with", "and",
    "for", "on", "is", "be", "or", "at", "as", "from", "that", "are", "it", "by",
    "all", "up", "like", "i", "just", "our", "use", "no", "an", "but", "we", "there",
    "too", "do", "have", "they"];

pub const ALIGN_TABS: bool = true;
