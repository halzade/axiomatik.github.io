#!/bin/bash

pkill -f "cargo run" || true
pkill -f "target/debug/axiomatik-web" || true

echo "shut down complete"