FROM rust AS build
WORKDIR /usr/src

RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new openlineplanner-backend
WORKDIR /usr/src/openlineplanner-backend
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN gcc --version
RUN TARGET_CC=gcc cargo install --target x86_64-unknown-linux-musl --path .

# Install osmtogeojson
RUN apk install npm
RUN npm install -g osmtogeojson

# FROM scratch
# COPY --from=build /usr/local/cargo/bin/openlineplanner-backend .
# USER 1000
CMD ["./openlineplanner-backend"]