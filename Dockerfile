FROM rust:1-slim AS build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:stable-slim
COPY --from=build /app/target/release/friendcine /friendcine
ENV PORT=8080
EXPOSE 8080
CMD ["/friendcine"]
