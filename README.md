# Do Everything Like a God - Developer Utilities

A high-performance, privacy-focused developer toolkit built with Rust and Leptos. All tools run client-side in WebAssembly.

## Features

- **Encoders**: Base64, Base32, Base58, HTML Escape, URL Escape
- **Convert**: Audio Converter (rodio decode to WAV)
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

## Support the Project

If this project has saved you time or helped you in your workflow, consider supporting its continued development. Your contribution helps me keep the project maintained and feature-rich!

[![][ko-fi-shield]][ko-fi-link]
[![][paypal-shield]][paypal-link]


<!-- Link Definitions -->
[ko-fi-shield]: https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white
[ko-fi-link]: https://ko-fi.com/kautism
[paypal-shield]: https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white
[paypal-link]: https://paypal.me/kautism

## License

MIT
