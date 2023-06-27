FROM node:lts-slim
# Install osmtogeojson
RUN npm install -g osmtogeojson
COPY ./target/x86_64-unknown-linux-musl/release/openlineplanner-backend .
USER 1000
CMD ["./openlineplanner-backend"]
