FROM node:lts-slim
# Install osmtogeojson
RUN npm install -g osmtogeojson
# COPY --from=builder ./target/x86_64-unknown-linux-musl/release/openlineplanner-backend .
COPY ./target/aarch64-unknown-linux-musl/release/openlineplanner-backend .
USER 1000
CMD ["./openlineplanner-backend"]
