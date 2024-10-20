# Stage 1: Install npm deps
FROM node:22-alpine AS frontend-deps
WORKDIR /app

COPY ./json-sharpener-web/*.json .
RUN npm install

# Stage 2: Builds the WASM library
FROM rust:1.81 AS wasm-build
WORKDIR /app

# Install wasm-pack
RUN cargo install wasm-pack

# Build WASM library
COPY ./json-sharpener ./json-sharpener
COPY ./json-sharpener-wasm ./json-sharpener-wasm
RUN wasm-pack build --target web ./json-sharpener-wasm

# Build the client
FROM frontend-deps AS frontend-and-wasm-build
WORKDIR /app

COPY ./json-sharpener-web/ .
COPY --from=wasm-build /app/json-sharpener-wasm/pkg/* ./src/assets/wasm/

# Build
RUN npm run build

# Stage 3: Serve the static files with NGINX
# TODO: this stage should probably be configured, now it's barebone / not production ready
FROM nginx:alpine
COPY --from=frontend-and-wasm-build /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]