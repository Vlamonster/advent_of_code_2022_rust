# How to run the solutions.
```shell
# Run all solutions.
cargo run --release

# Run the solution for both parts of a day.
cargo run --release -- 1

# Run the solution for part 1 or 2 of a day.
cargo run --release -- 1 1
cargo run --release -- 1 2

# Run the solution with path to input file (day must be specified).
cargo run --release -- 1 -i /inputs/d01_example.txt

# If you would like to make changes, you can verify they are correct.
cargo test
```

*Some of the solutions are still in the stand-alone branch. Refer to those if it is missing here.*