# JSON Sharpener {⚡️}

[![Build and Tests](https://github.com/larsjuvik/json-sharpener/actions/workflows/CI.yml/badge.svg)](https://github.com/larsjuvik/json-sharpener/actions/workflows/CI.yml)
[![Deployment](https://github.com/larsjuvik/json-sharpener/actions/workflows/CD.yml/badge.svg)](https://github.com/larsjuvik/json-sharpener/actions/workflows/CD.yml)

Sharpen your JSON files with `json-sharpener` - a `library`/`CLI-tool`/`web-app` for easy JSON to `C#` converison.

![Screenshot of the web application](./res/screenshot-web-app.png)

## Folders :file_folder:

- `json-sharpener` - the core library for parsing JSON to C#
- `json-sharpener-terminal` - terminal parser that uses the core library
- `json-sharpener-wasm` - a WebAssembly wrapper around the core library
- `json-sharpener-web` - website that uses the WebAssembly core library

## Run :rocket:

### Terminal Application

```bash
cargo build --release -p json-sharpener-terminal  # build
./target/release/json-sharpener-terminal -f YOUR_FILE_HERE
```

If you don't want to build the program and then run it in separate steps,
you can use this one-liner:

```bash
cargo run --release -- -f YOUR_TEST_FILE.json
```

### Web Application

#### Docker

```bash
docker build -t json-sharpener-web .
docker run -p 8080:80 json-sharpener-web
```

#### Without Docker

To build the WASM library (used in web app), make sure you have Rust compiler and `wasm-pack` installed.

```bash
cd ./json-sharpener-web/
npm run build-wasm && npm run dev
```
