# JSON Sharpener

[![Build and Tests](https://github.com/larsjuvik/json-sharpener/actions/workflows/CI.yml/badge.svg)](https://github.com/larsjuvik/json-sharpener/actions/workflows/CI.yml)

Sharpen your JSON files with `json-sharpener` - a CLI tool for easy conversions to `C#` classes.

## Build :hammer:

```bash
cargo build --release
```

## Run :rocket:

If you don't want to build the program and then run it in separate steps,
you can use this one-liner:

```bash
cargo run --release -- -f YOUR_TEST_FILE.json
```
