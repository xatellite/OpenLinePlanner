FROM rust AS build
ARG TARGETPLATFORM

WORKDIR /usr/src
RUN export ARCH=$(echo "$TARGETPLATFORM" | sed "s/linux\///" | sed "s/arm64/aarch64/" | sed "s/amd64/x86_64/") && rustup target add ${ARCH}-unknown-linux-musl

RUN USER=root cargo new openlineplanner-backend
WORKDIR /usr/src/openlineplanner-backend
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN gcc --version
RUN export ARCH=$(echo "$TARGETPLATFORM" | sed "s/linux\///" | sed "s/arm64/aarch64/" | sed "s/amd64/x86_64/") && TARGET_CC=gcc cargo install --target $ARCH-unknown-linux-musl --path .

FROM node:lts-slim
# Install osmtogeojson
RUN npm install -g osmtogeojson
COPY --from=build /usr/local/cargo/bin/openlineplanner-backend .
USER 1000
CMD ["./openlineplanner-backend"]
