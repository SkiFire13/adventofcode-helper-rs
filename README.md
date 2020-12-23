Helper crate for solving the Advent of Code.

### Setup your crate

Your `main.rs` should look like the following:

```rust
aoc_helper::main!($YEAR =>
    day1,
    day2,
    // ...
); 
```

Here `$YEAR` is the year of the AoC you will be solving in this crate. There's no support for multiple years in the same crate.

`day1`, `day2` ecc ecc are the days you've solved. Each day must be solved in a `dayN` module which exposes the following functions:

   * `pub fn input_generator(input: &str) -> Input` where:
      * `input` is the trimmed version of the input file contents
      * `Input` can be a custom type as long as it's consistent inside the same day module
      * `Input` can borrow `input`
   * `pub fn part1(input: &Input) -> Part1Answer` where:
      * `Input` should be the same as the `input_generator` one
         * Technically you could swap it with any type that can be deref coerced from `&Input`, however it's preferred to keep `&Input` for faster typing and more uniform solutions. It's suggested to add `#![allow(clippy::ptr_arg)]` to `main.rs` to silence possible warnings derived from this.
      * `Part1Answer` is any type that implements `Display`
   * `pub fn part2(input: &Input) -> Part2Answer` where:
      * `&Input` has the same restrictions as in `part1`
      * `Part2Answer` is any type that implements `Display`
      * It's not required for `day25`, in which case it will be ignored if present.

You can have the macro setup each `dayN.rs` by running the following command:

```sh
cargo run -- setup $DAY
```
    
The module structure is hardcoded in this library and can't be customized in any way other than forking the library itself. This is the price for having a fast declarative macro doing all the work for you.

### Prelude

This crate offers an opinionated prelude which is also re-exported when calling the `main!` macro. You can find it in `src/prelude.rs`

### Setup the AoC credentials

Find your session token for the AoC. You'll need to visic adventofcode.com open the devtools of your browser, then:

   * Firefox: "Storage" tab, "Cookies" folder, and copy the "Value" field of the "session" cookie.
   * Google Chrome / Chromium: "Application" tab, "Cookies" folder, and copy the "Value" field of the "session" cookie.

Then run the following command:

```sh
cargo run -- session $SESSION
```

### Running the solutions for the latest day

```sh
cargo run --release
```

Note: this will download the input file if it's missing

### Running the solutions for the a specific day

```sh
cargo run --release -- -d $DAY
```

Note: this will download the input file if it's missing

### Running all the solutions

```sh
cargo run --release -- -d all
```

Note: this will download all the missing input files

### Download the input file for today

```sh
cargo run -- input
```

### Download the input file for all days

```sh
cargo run -- input -d all
```

### Download the input file for a specific day

```sh
cargo run -- input -d $DAY
```

