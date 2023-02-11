#!/usr/bin/env just --justfile

run:
    cargo run --bin fanyi

release:
  cargo build --release    

install:
    cargo install --path .

lint:
  cargo clippy

example:
  cargo run --example exname -- arg1