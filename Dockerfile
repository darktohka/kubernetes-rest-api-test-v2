# Use xx for cross-compiling
FROM --platform=$BUILDPLATFORM tonistiigi/xx AS xx

# Use the official Rust image for building
FROM --platform=$BUILDPLATFORM rust:alpine as builder
COPY --from=xx / /

RUN apk add clang lld

ARG TARGETPLATFORM
WORKDIR /srv

COPY ./Cargo.toml ./Cargo.lock /srv/
COPY ./src /srv/src

RUN xx-cargo build --release

# Build the actual image
FROM debian:trixie-slim

WORKDIR /app
COPY --from=builder /srv/target/release/restapp .

EXPOSE 5001
CMD ["./restapp"]
