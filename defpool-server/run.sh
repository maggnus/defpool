#!/bin/bash
cd "$(dirname "$0")"
RUST_LOG=info ./target/release/defpool-server --config defpool-server.toml
