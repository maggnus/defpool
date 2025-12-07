FROM lukemathwalker/cargo-chef:latest-rust-1.83 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
WORKDIR /app/defpool-proxy
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/defpool-proxy/recipe.json recipe.json
# Copy contrib to /app/contrib (expected relative path)
COPY contrib /app/contrib
# Copy contrib to /contrib (fallback for absolute path resolution issue)
COPY contrib /contrib

# Build dependencies
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
WORKDIR /app/defpool-proxy
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/defpool-proxy/target/release/defpool-proxy /app/defpool-proxy
COPY --from=builder /app/defpool-proxy/defpool-proxy.toml /app/defpool-proxy.toml
EXPOSE 3333
CMD ["./defpool-proxy", "--config", "defpool-proxy.toml"]
