#!/bin/bash
cargo fmt --all -- --check
cargo clippy --all -- -D warnings
cargo test --all
cargo build --all --release
cargo run