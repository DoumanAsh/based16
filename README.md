# based16

[![Rust](https://github.com/DoumanAsh/based16/actions/workflows/rust.yml/badge.svg)](https://github.com/DoumanAsh/based16/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/based16.svg)](https://crates.io/crates/based16)
[![Documentation](https://docs.rs/based16/badge.svg)](https://docs.rs/crate/based16/)

Simple HEX encoder/decoder for Rust chads

## Implementation notes

- SSE2 implemented as simplest and most widely available HW acceleration
- Everything else is too much pain in ass for me to do, but PRs are welcome
