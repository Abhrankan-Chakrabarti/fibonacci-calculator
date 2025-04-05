# Fibonacci Calculator

A fast and efficient command-line tool written in Rust for calculating large Fibonacci numbers. Supports decimal, scientific notation, and hexadecimal input formats.

## Features

- Computes Fibonacci numbers using an optimized recursive algorithm
- Accepts input in:
  - Decimal (e.g., `100`)
  - Scientific notation (e.g., `1e2`)
  - Hexadecimal (e.g., `0x64`)
- Optional flag to suppress printing the result
- Displays computation time

## Installation

Clone the repository and build it using Cargo:

```bash
git clone https://github.com/Abhrankan-Chakrabarti/fibonacci-calculator.git
cd fibonacci-calculator
cargo build --release
```

## Usage

```bash
cargo run --release -- [OPTIONS]
```

## Options

- --no-print : Suppress printing the Fibonacci result

- --help     : Show help message

- --version  : Show version information


## Example

```bash
cargo run --release
```

```
This program computes the n-th Fibonacci number.
Enter n :    1e3
F(1000)      = 434665576869374564356885276750406258025646605173717804024817290895...
Elapsed time = 0.000021 sec
```

## Author

Abhrankan Chakrabarti

## License

This project is licensed under the MIT License.
