FROM node:22-alpine AS base

WORKDIR /app

# Dependencies
COPY ./json-sharpener-web/package.json /json-sharpener-web/package-lock.json ./
RUN npm install

COPY ./json-sharpener-web .
COPY ./json-sharpener-wasm/ ./json-sharpener-wasm
ENV NEXT_TELEMETRY_DISABLED=1

# Build
RUN npm run build

# TODO: add wasm

# Expose the Next.js default port
EXPOSE 3000

# Start the Next.js server
CMD ["npm", "run", "start"]
