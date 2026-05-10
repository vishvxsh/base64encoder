# Workflow: Building a Base64 Encoder in Rust

This workflow breaks down the approved implementation plan into actionable steps to build the Base64 encoder and decoder in Rust.

## Step 1: Set Up the Project Structure

1. Ensure we are in the `base64encoder` Cargo project directory.
2. The `Cargo.toml` is already created. We will only use standard library functionalities, so no external dependencies need to be added to `Cargo.toml`.
3. Create a new file `src/base64.rs` to contain the core encoding and decoding logic.

## Step 2: Implement the Base64 Encoding Logic

In `src/base64.rs`, we will write the `encode` function.

1. **Define the Alphabet**: Define a constant array `ALPHABET` containing the 64 characters used in Base64 (`A-Z, a-z, 0-9, +, /`).
2. **Process in Chunks**: The `encode` function takes a byte slice (`&[u8]`). It will iterate through the input bytes in chunks of 3.
3. **Bitwise Operations**:
   - For each 3-byte chunk (24 bits total), we'll shift the bytes to form a single 24-bit integer (`u32`).
   - Extract four 6-bit values from this integer using bitwise `&` (AND) and `>>` (right shift) operations.
   - Use these 6-bit values as indices to look up characters in the `ALPHABET`.
4. **Handle Padding**: 
   - If the input length isn't perfectly divisible by 3, the final chunk will have 1 or 2 bytes.
   - For a 2-byte chunk, add one padding character (`=`).
   - For a 1-byte chunk, add two padding characters (`==`).
5. **Write Unit Tests**: Add basic tests directly in `src/base64.rs` using the `#[cfg(test)]` module, testing against standard RFC 4648 test vectors (e.g., `"" -> ""`, `"f" -> "Zg=="`).

## Step 3: Implement the Base64 Decoding Logic

In `src/base64.rs`, we will write the `decode` function.

1. **Create Reverse Lookup**: To efficiently decode, we can create a reverse lookup table (an array mapping ASCII character values to their 6-bit Base64 values).
2. **Filter Input**: The input string might contain newlines or spaces. We should ideally ignore whitespaces.
3. **Process in Chunks of 4**: The `decode` function takes a string slice (`&str`). It processes characters in groups of 4.
4. **Reverse Bitwise Operations**:
   - Accumulate four 6-bit values to form a 24-bit integer.
   - Extract up to three 8-bit bytes from this integer.
5. **Handle Padding**: Adjust the number of extracted bytes based on whether the input group ended with one or two `=` characters.
6. **Write Unit Tests**: Add decoding tests in `src/base64.rs`.

## Step 4: Build the Command Line Interface (CLI)

Update `src/main.rs` to expose the library to the user.

1. **Module Declaration**: Add `mod base64;` to `main.rs`.
2. **Argument Parsing**: Read `std::env::args()`.
3. **Subcommands**: Check the first argument to determine the action: `encode` or `decode`.
4. **Input Handling**:
   - If a string is provided as the second argument, encode/decode it directly.
   - If no second argument is provided, read all bytes from standard input (`std::io::stdin().read_to_end(...)`).
5. **Output**: Print the result to standard output.

## Step 5: Verification and Final Checks

1. Run `cargo test` to execute all unit tests.
2. Run manual tests:
   ```bash
   echo -n "Hello, World!" | cargo run -- encode
   cargo run -- decode "SGVsbG8sIFdvcmxkIQ=="
   ```

---

We will proceed to implement these steps one by one. I will start by writing `src/base64.rs`.
