# GlideRS

[![Crates.io](https://img.shields.io/crates/v/gliders.svg?style=for-the-badge&logo=crates.io)](https://crates.io/crates/gliders)
[![Docs.rs](https://img.shields.io/badge/docs.rs-gliders-blue?style=for-the-badge&logo=docs.rs)](https://docs.rs/gliders)
[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-7F0000?style=for-the-badge&labelColor=000000&logoColor=white)](LICENSE)
[![Support on Ko-fi](https://img.shields.io/badge/ko--fi-Donate-999999?style=for-the-badge&logo=ko-fi&labelColor=333333)](https://ko-fi.com/stacksyndicate)

This crate is nowhere near ready for actual use.

## What?

It's my attempt at building my ideal programming language by leveraging another programming language that already comes painfully close. Expect constant breakages and long-term updates.

## How?

This language is built mainly on *pest* and *syn*. In the future I'll use a crate for building a language server.

### Compiler

```bash
cargo install gliders
```

### Macro

```bash
cargo add gliders
```

## Why?

This started out as a few useful macros and helper functions that I thought might be useful to someone else somewhere. Then I started to realise that I was really forcing Rust not to be so low level.
