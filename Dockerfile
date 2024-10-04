# Stage 1: First stage builds the WASM library
FROM rust:1.81 AS rust-base
WORKDIR /app

# Install wasm-pack
RUN cargo install wasm-pack

# Build WASM library
COPY ./json-sharpener ./json-sharpener
COPY ./json-sharpener-wasm ./json-sharpener-wasm
RUN wasm-pack build --target web ./json-sharpener-wasm


# Stage 2: Build Next.js application and include WASM library
FROM node:22-alpine AS base
WORKDIR /app

# Dependencies for Next.js app
COPY ./json-sharpener-web/*.json ./
RUN npm install

COPY ./json-sharpener-web/ .
COPY --from=rust-base /app/json-sharpener-wasm/pkg ./public/wasm

# Build
ENV NEXT_TELEMETRY_DISABLED=1
RUN npm run build

# Expose the Next.js default port
EXPOSE 3000

# Start the Next.js server
CMD ["npm", "run", "start"]
