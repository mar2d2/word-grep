# word-grep
CLI that does some word count and searches

## Usage
``` 
USAGE:
    grep-lite.exe [OPTIONS] <input>

ARGS:
    <input>    File to search

OPTIONS:
        --count-lines                Counts lines in file
        --count-words                Counts the words in file
    -h, --help                       Print help information
    -i, --ignore-case                ignore caseing in word occurrence
    -p, --pattern <PATTERN>          The pattern to search for
    -V, --version                    Print version information
        --word-occurrences <WORD>    Counts the occurrences of a given word in file
```

## Examples

### Search for occurrences of 'scooby' in file
```
grep-lite.exe --word-occurrences scooby -i quote.txt
```

### Find all lines that contain 'scooby'
```
grep-lite.exe --pattern Scooby ../../quote.txt
```
