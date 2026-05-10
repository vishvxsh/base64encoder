# Base64 Encoder Implementation Plan

This plan outlines the approach to building a standalone Base64 encoder (and decoder) in Rust from scratch, without relying on external crates for the encoding logic.

## Goal Description

We want to create a `base64encoder` command-line application in Rust that can encode and decode base64 strings. The implementation will demonstrate bitwise operations by mapping 3-byte chunks to 4 printable characters using the standard Base64 alphabet.

## Proposed Changes

### Core Logic Module

We will create a module (e.g., `src/base64.rs`) to hold the encoding and decoding logic.

#### [NEW] `src/base64.rs`
- **Constant:** Define the Base64 alphabet: `b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"`.
- **Encode function (`pub fn encode(data: &[u8]) -> String`):**
  - Allocate a `String` with capacity `(data.len() + 2) / 3 * 4`.
  - Iterate over the input in chunks of 3 bytes.
  - For full 3-byte chunks: shift bytes into a 24-bit integer, and extract four 6-bit values to index the alphabet.
  - For 2-byte leftovers: shift into a 24-bit integer, extract three characters, and add a single `=` padding.
  - For 1-byte leftovers: shift into a 24-bit integer, extract two characters, and add two `=` paddings.
- **Decode function (`pub fn decode(encoded: &str) -> Result<Vec<u8>, String>`):**
  - Implement the reverse mapping. Ignore or fail on whitespace.
  - Handle valid padding appropriately.

### Command Line Interface

We will update the main.rs file to provide a simple command-line interface.

#### [MODIFY] `src/main.rs`
- Import the `base64` module.
- Parse command-line argumets (e.g., using standard `std::env::args`).
  - Support subcommands: `encode` and `decode`.
  - Accept input either from an argument or from stdin.
  - Print the output to stdout.

## User Review Required

> [!NOTE]
> We are planning to build the Base64 algorithm from scratch (standard RFC 4648) to demonstrate how it works.

> [!TIP]
> Are there specific variations of Base64 you want to support (e.g., URL-safe base64 `-_` instead of `+/`)?

## Verification Plan

### Automated Tests
- Unit tests in `src/base64.rs` using standard test vectors (e.g., RFC 4648 examples):
  - `""` -> `""`
  - `"f"` -> `"Zg=="`
  - `"fo"` -> `"Zm8="`
  - `"foo"` -> `"Zm9v"`
  - `"foob"` -> `"Zm9vYg=="`
  - `"fooba"` -> `"Zm9vYmE="`
  - `"foobar"` -> `"Zm9vYmFy"`

### Manual Verification
- We will build the CLI tool and manually run it against random byte strings (using piping like `echo "hello" | cargo run -- encode` and checking against the system `base64` command).
