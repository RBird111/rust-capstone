#!/usr/bin/env bash
export NODE_OPTIONS=--openssl-legacy-provider
npm install --prefix frontend
npm run build --prefix frontend
cargo build --release
cargo install diesel_cli --no-default-features --features postgres
diesel database setup
cargo run --release --bin seed_all
