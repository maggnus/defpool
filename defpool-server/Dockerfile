FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY defpool-server defpool-server
WORKDIR /app/defpool-server
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/defpool-server/recipe.json recipe.json

# Build dependencies
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY defpool-server .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/defpool-server /app/defpool-server
COPY --from=builder /app/defpool-server.toml /app/defpool-server.toml
EXPOSE 3000
CMD ["./defpool-server", "--config", "defpool-server.toml"]
