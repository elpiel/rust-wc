# Word count written in Rust

## How to build:

* You need Rust install, so head to [Rust-lang.org](https://www.rust-lang.org/) and follow the instructions
* Clone the repository and `cd` in the folder
* Run `cargo build --release`
* In command line you can run the release binary:

```
./target/release/wc target-file-path
```

* To test if everything is working, you can test with the `whitespace` text file in this project:

```
./target/release/wc whitespace
```