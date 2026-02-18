# Do Everything Like a God - Developer Utilities

A high-performance, privacy-focused developer toolkit built with Rust and Leptos. All tools run client-side in WebAssembly.

## Features

- **Encoders**: Base64, Base32, Base58, HTML Escape, URL Escape
- **Cryptography**: Hash (MD5, SHA-1, SHA-256, SHA-512), AES Encryption, JWT Decoder
- **Development**: JSON Formatter, Regex Tester, Diff Checker, UUID Generator, Timestamp Converter, Base Converter, Cron Parser, Image to Base64

## Tech Stack

- Rust + WebAssembly
- Leptos (SPA framework)
- Trunk (build tool)

## Build

```bash
cargo install trunk
trunk serve
```

## Deploy

```bash
trunk build --release
```

## Development

```bash
# Run tests
cargo test

# Lint
cargo fmt --check
cargo clippy -- -D warnings
```

## License

MIT
