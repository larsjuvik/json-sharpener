# JSON Sharpener

[![Build and Tests](https://github.com/larsjuvik/json-sharpener/actions/workflows/CI.yml/badge.svg)](https://github.com/larsjuvik/json-sharpener/actions/workflows/CI.yml)

Sharpen your JSON files with `json-sharpener` - a CLI tool for easy conversions to `C#` classes.

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

```bash
docker build -t json-sharpener-web .
docker run -p 3000:3000 json-sharpener-web
```
