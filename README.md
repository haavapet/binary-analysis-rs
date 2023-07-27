# 🦀 binary-analysis-rs 🚀

![Build Status](https://github.com/haavapet/binary-analysis-rs/actions/workflows/pipeline.yml/badge.svg)

📝 Short description: binary-analysis-rs 🚀 is a Rust-based project that facilitates the analysis of binaries from unknown instruction set architectures. It accepts a binary file and several optional parameters, such as instruction length, to identify the most likely opcodes along with an associated probability rating.

## Features

🔍 Key Features:

- Accurate Analysis: Identify the most likely call and return opcodes with a high degree of certainty.
- Probability Metric: The tool utilizes a probability metric to prioritize the most promising results.
- Custom Parameters: Users can input optional parameters like instruction length and endianness for tailored analysis.
- Parallellization: Support for parallellization using the `--parallell` parameter
- Speed: Faster than the [Python implementation](https://github.com/haavapet/binary-analysis) by ~100x. (10 minutes vs 6 seconds for a 50mb binary on a 5 year old intel laptop). The program has been heavily optimized using tools such as [HeapTrack](https://github.com/KDE/heaptrack) and [FlameGraph](https://github.com/flamegraph-rs/flamegraph).

## Installation

To install and use the Binary Analysis Tool, follow these straightforward steps:

1. Clone the repository to your local machine.
2. Ensure you have Rust installed on your system. If not, download it from [Rust's official website](https://www.rust-lang.org/).
3. Navigate to the project directory using the terminal.
4. Build the project by running `cargo build`.
5. Execute the program with your binary file and desired parameters using `cargo run`.

## Usage

The Binary Analysis Tool is simple and intuitive to use. Two example usages of the program are shown below.

`cargo run --release -- <Path>/openvpn_mips -i 32 -c 6 --endiannes big --left-shift-call-operand 2 --addressing-mode absolute`

_Finds correct with probability 0.62_

`cargo run --release -- <Path>/ffmpeg_aarch64 -i 32 -c 6 --endiannes little --left-shift-call-operand 2 --addressing-mode relative`

_Finds correct with probability 0.8_

To show a list of all parameters, run the program with the `--help` flag.

## How it Works

The Binary Analysis Tool operates by employing a heuristic approach based on opcode frequency and operand inspection. By analyzing the instruction length and endianness, the tool identifies the most likely call and return opcodes and calculates their probability rating. See the associated research paper for a more thorough analysis.

## License

This project is licensed under the MIT License. For more details, see the [LICENSE](LICENSE) file.
