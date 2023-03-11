#!/usr/bin/env just --justfile

hello:
  echo "hello world"

# run app
dev:
  cargo tauri dev

# build app
build:
  cargo tauri build