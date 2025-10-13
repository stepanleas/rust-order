FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release
COPY . .
RUN cargo build -p starter --release
RUN mv ./target/release/starter ./app

FROM debian:stable-slim AS runtime
RUN apt-get update && apt-get install -y \
    libpq5 \
    curl \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/app /usr/local/bin/
COPY ./configuration ./configuration
ENTRYPOINT ["/usr/local/bin/app"]