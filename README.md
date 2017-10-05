# Word count written in Rust

## How to build:

* You need Rust installed, so first head to [Rust-lang.org](https://www.rust-lang.org/) and follow the instructions
* Clone the repository and `cd` in to the folder
* Run `cargo build --release`
* Now you can run the release binary:

```
./target/release/wc target-file-path
```

* To test if everything is working, you can test with the `whitespace` text file in this project:

```
./target/release/wc whitespace
```