# GlideRS

[![Crates.io](https://img.shields.io/crates/v/gliders.svg)](https://crates.io/crates/gliders)
[![Docs.rs](https://docs.rs/gliders/badge.svg)](https://docs.rs/gliders)
[![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=red)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/license/mit/)

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

This started out as just a few useful macros and helper functions that I thought might be useful to someone else somewhere. Then I started to realise that I was really forcing Rust not to be so verbose or specific about types, among other things.
