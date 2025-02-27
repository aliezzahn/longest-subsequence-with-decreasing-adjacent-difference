# Longest Subsequence with Decreasing Adjacent Difference

A Rust library to find the length of the longest subsequence with decreasing adjacent differences.

![CI](https://github.com/aliezzahn/longest-subsequence-with-decreasing-adjacent-difference/actions/workflows/ci.yml/badge.svg)
![CD](https://github.com/aliezzahn/longest-subsequence-with-decreasing-adjacent-difference/actions/workflows/cd.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

Given a sequence of integers, this library computes the length of the longest subsequence where the difference between adjacent elements is strictly decreasing.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
longest-subsequence-with-decreasing-adjacent-difference = "0.1"
```

## Usage

```rust
use longest_subsequence_with_decreasing_adjacent_difference::Solution;

fn main() {
    let nums = vec![10, 20, 10, 19, 10, 20];
    let result = Solution::longest_subsequence(nums);
    println!("Length of the longest subsequence: {}", result);
}
```

## Example Output

For the input `[10, 20, 10, 19, 10, 20]`, the output will be:

```
Length of the longest subsequence: 5
```

## Running Tests

To run the tests, use the following command:

```bash
cargo test
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on [GitHub](https://github.com/aliezzahn/longest-subsequence-with-decreasing-adjacent-difference).

## Contact

For questions or feedback, feel free to reach out:

- Email: [aliezzahn@gmail.com](mailto:aliezzahn@gmail.com)
- GitHub: [aliezzahn](https://github.com/aliezzahn)
