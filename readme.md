# Overview

Compile the code to WebAssembly. Prepare JS/TS frontend and create browser-based Snake game


## Installing

## Rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

## env
source "$HOME/.cargo/env"

## Dependencies
- wasm-bindgen

## Libs

- cdylib

## Generating wasm 

```bash
~ cargo install wasm-pack
```

```bash
~ wasm-pack build --target web
```

# Learning 

Base class for training: https://www.udemy.com/course/rust-webassembly-with-js-ts-the-practical-guide


