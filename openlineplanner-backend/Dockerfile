FROM rust:latest as builder
COPY Cargo.toml .
COPY Cargo.lock .
COPY src/ ./src/
RUN apt update && apt install -y musl musl-tools
RUN which musl-gcc
RUN rustup target add aarch64-unknown-linux-musl
RUN cargo build --target aarch64-unknown-linux-musl --release

FROM node:lts-slim
# Install osmtogeojson
RUN npm install -g osmtogeojson
COPY --from=builder ./target/aarch64-unknown-linux-musl/release/openlineplanner-backend .
USER 1000
CMD ["./openlineplanner-backend"]
