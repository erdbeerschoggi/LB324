FROM rust:slim AS build
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown && cargo install trunk --version 0.21.14 --locked
WORKDIR /app
COPY . .
RUN cargo build --release --locked -p rust-rechner-backend
RUN cd Frontend && trunk build --release --locked

FROM debian:stable-slim
LABEL org.opencontainers.image.source="https://github.com/erdbeerschoggi/LB324"
RUN useradd --system --uid 10001 app
WORKDIR /app
COPY --from=build /app/target/release/rust-rechner-backend /app/server
COPY --from=build /app/Frontend/dist /app/public
ENV STATIC_DIR=/app/public PORT=3000
USER app
EXPOSE 3000
CMD ["/app/server"]
