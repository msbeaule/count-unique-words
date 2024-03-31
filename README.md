# Count Unique Words

Counts how many times each word appears in text files.

In `main.rs`, you can specify what words to skip, to only show the words that have a certain minimum of hits, or only show the first x number of words.

## Run

Single file (can be any extension):

```
cargo run -- <file.txt>
```

Example:

```
cargo run -- words.txt
```

Current folder and all subfolders in that folder (currently only searches for `.md` files):

```
cargo run -- -d .
```
