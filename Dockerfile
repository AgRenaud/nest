FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app
RUN apt update && apt upgrade -y && apt install lld clang -y

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin nest

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get upgrade -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/nest nest

COPY config.default.toml config.default.toml
COPY templates/ templates/
COPY static/ static/

ENTRYPOINT ["./nest"]
EXPOSE 5037
