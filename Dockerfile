# Use the official Rust image for building
FROM rust:latest as builder

WORKDIR /srv
COPY ./Cargo.toml ./Cargo.lock /srv/
COPY ./src /srv/src

RUN cargo build --release

# Build the actual image
FROM debian:trixie-slim

WORKDIR /app
COPY --from=builder /srv/target/release/restapp .

EXPOSE 5001
CMD ["./restapp"]
