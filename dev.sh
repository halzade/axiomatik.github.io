#!/bin/bash

pkill -f "cargo run" || true
pkill -f "target/debug/axiomatik-web" || true

export APP_ENVIRONMENT=dev

cargo run -- delete-user lukas
cargo run -- create-user lukas dev

cargo watch -x run
