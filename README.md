# Advent of Code 2021

My solutions of Advent of Code 2022

https://adventofcode.com/2022

## Utility scripts

### `add_day.sh`

- Creates a new source file `src/bin/day_${day_number}_${title}.rs`
- Creates an empty test input file `test_inputs/day_${day_number}_${title}.txt`
- Creates an empty input file `inputs/day_${day_number}_${title}.txt`

### `test_day.sh`

- Executes the program for given day on test input data

### `run_day.sh`

- Executes the program for given day on input data


---

# Code snippets


### Read lines of numbers
```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Cannot read file");
    let numbers = contents.lines().map(|v| v.parse::<i64>().unwrap());
}
```


### TODO:
 - Put together utility functions for working with intervals
 