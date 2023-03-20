#!/usr/bin/env just --justfile

hello:
  echo "hello world"

# run app
dev:
  RUST_BACKTRACE=1 cargo tauri dev

# build app
build:
  cargo tauri build

install: build
  mv ./src-tauri/target/release/popup-translation ~/.cargo/bin/