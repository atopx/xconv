# xconv

![Crates.io msrv](https://img.shields.io/crates/msrv/xconv)
![License](https://img.shields.io/badge/license-MIT-blue)


`xconv` is a high-performance batch file encoding conversion tool written in Rust. It supports converting files or directories from one encoding to another, with a focus on speed and efficiency.

## Features

- **Batch Conversion**: Convert multiple files or entire directories at once.
- **High Performance**: Utilizes Rust's concurrency features to achieve maximum performance.
- **Flexible Input and Output**: Specify input and output as files or directories.
- **Easy to Use**: Simple command-line interface.

## Installation

You can install `xconv` using `cargo`, the Rust package manager:

```sh
cargo install xconv
```


## Usage

```sh
xconv -i <input_path> -o <output_path> -f <source_encoding> -t <target_encoding>
```

## Examples


### Convert a Single File

```sh
xconv -i /path/to/input_file.txt -o /path/to/output_file.txt -f UTF-8 -t GBK
```


### Convert All Files in a Directory

```sh
xconv -i /path/to/input_dir -o /path/to/output_dir -f UTF-8 -t GBK
```

## Command-Line Options

- -i, --input: Path to the input file or directory.
- -o, --output: Path to the output file or directory.
- -f, --from: Source encoding.
- -t, --to: Target encoding.


## License

`xconv` is licensed under the MIT license. See [LICENSE](LICENSE) for more details.


## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.
