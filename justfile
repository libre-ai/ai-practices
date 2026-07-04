#!/usr/bin/env just --justfile

# Single-origin local serve: build the Dioxus web bundle and run the API server.
serve-local:
    dx build --platform web --release
    cargo run -p cli -- serve --web-root target/dx/rumble-ai-practices-web-app/release/web/public
