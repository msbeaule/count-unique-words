# Count Unique Words

Counts how many times each word appears in text or markdown files.

Make a copy of `example-options.rs` and rename to `options.rs`. In this file, you can specify what words to skip, to only show the words that have a certain minimum of hits, or only show the first x number of words.

Make a copy of `example-words.rs` and rename to `words.rs` and list the words you would like to be grouped together when searching. Some words are included as an example.

## Run

### Searching in a single file (any extension)

```
cargo run -- <file.txt>
```

Example:

```
cargo run -- words.txt
```

### Searching in a directory

Current folder and all subfolders in that folder (currently only searches for `.md` files):

```
cargo run -- -d .
```
