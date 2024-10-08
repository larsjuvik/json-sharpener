# JSON Sharpener

[![Build and Tests](https://github.com/larsjuvik/json-sharpener/actions/workflows/CI.yml/badge.svg)](https://github.com/larsjuvik/json-sharpener/actions/workflows/CI.yml)

Sharpen your JSON files with `json-sharpener` - a CLI tool for easy conversions to `C#` classes.

## Overview

- `json-sharpener` - the core library for parsing JSON to C#
- `json-sharpener-terminal` - terminal parser that uses the core library
- `json-sharpener-wasm` - a WebAssembly wrapper around the core library
- `json-sharpener-web` - website that uses the WebAssembly core library

## Run :rocket:

### Terminal

```bash
cargo build --release -p json-sharpener-terminal  # build
./target/release/json-sharpener-terminal -f YOUR_FILE_HERE
```

If you don't want to build the program and then run it in separate steps,
you can use this one-liner:

```bash
cargo run --release -- -f YOUR_TEST_FILE.json
```

### Web

#### Docker

```bash
docker build -t json-sharpener-web .
docker build --platform=linux/amd64 -t json-sharpener-web .  # if amd64 needed
docker run -p 3000:3000 json-sharpener-web
```

#### Without Docker

```bash
wasm-pack build --target web ./json-sharpener-wasm
mkdir -p ./json-sharpener-web/public/wasm/
cp ./json-sharpener-wasm/pkg/*.js ./json-sharpener-web/public/wasm/
cp ./json-sharpener-wasm/pkg/*.ts ./json-sharpener-web/public/wasm/
cp ./json-sharpener-wasm/pkg/*.wasm ./json-sharpener-web/public/wasm/
```
