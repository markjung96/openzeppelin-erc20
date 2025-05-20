# OpenZeppelin ERC20 Implementation with Stylus

This repository contains an implementation of the ERC20 token standard using OpenZeppelin's Stylus SDK. The implementation is based on the example from [OpenZeppelin's Rust Contracts for Stylus](https://github.com/OpenZeppelin/rust-contracts-stylus/tree/main/examples/erc20).

## Development Environment

- Operating System: macOS
- Stylus SDK Version: 0.5.12
- Rust Edition: 2021

## Project Overview

This project is part of an effort to integrate Stylus contract development into Remix IDE, allowing developers to compile and deploy Stylus contracts directly from the browser interface.

## Current Issues

We are currently experiencing some challenges with the `cargo stylus export-abi` command:

1. The command internally calls `cargo run`
2. When executing, we encounter errors related to the OpenZeppelin Stylus SDK
3. Main error types include:
   - Issues with `std` module resolution
   - Dependency conflicts

## Development Status

- [x] Initial implementation using OpenZeppelin's example
- [x] Basic project setup
- [ ] Successful ABI generation
- [ ] Integration with Remix IDE

## Next Steps

Working on resolving the `cargo stylus export-abi` issues to enable seamless integration with Remix IDE.

## Contributing

If you've encountered similar issues or have suggestions for solutions, please feel free to open an issue or submit a pull request.
