#!/bin/bash

pkill -f "cargo run" || true
pkill -f "target/debug/axiomatik-web" || true

echo "shut down complete"

export APP_ENVIRONMENT=dev

echo "starting"
cargo run
