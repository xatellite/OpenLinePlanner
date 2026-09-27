FROM node:lts-slim
# COPY --from=builder ./target/x86_64-unknown-linux-musl/release/openlineplanner-backend .
COPY ./target/aarch64-unknown-linux-musl/release/openlineplanner-backend .
USER 1000
CMD ["./openlineplanner-backend"]
