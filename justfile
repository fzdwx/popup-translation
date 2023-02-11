#!/usr/bin/env just --justfile

run args: release
    ./target/release/fanyi {{args}}


release:
  cargo build --release

install:
    cargo install --path .

lint:
  cargo clippy

example:
  cargo run --example exname -- arg1