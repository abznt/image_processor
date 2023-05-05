# Compilation
**NOTE:** this project requires OpenImageIO as a pre-requisite. It can be installed on Debian based systems with `sudo apt install libopenimageio-dev`
```sh
cargo build --release
```

# Execution
A help message detailing the command-line arguments with
```sh
cargo run -- --help
```
An example execution that increases the brightness of an image by 50.0 using 6 threads:
```sh
cargo run --release -- resources/fruit.jpg target/test.jpg -j6 brightness -- 50
```

# Benchmarking
Benchmarking can be run for [1, 2, 4, 8] cores with
```sh
cargo bench
```
Results are written to `target/criterion/report/index.html`