#!/bin/bash


## Taken from https://dev.to/petr7555/github-action-for-multi-project-rust-repository-28d5
## Thanks to https://dev.to/petr7555

build_test_clippy(){
  while read path; do
    printf "Project: %s\n" "$path"
    cargo build --verbose --manifest-path "$path"
    cargo test --verbose --manifest-path "$path"
    cargo clippy --verbose --manifest-path "$path"
  done
}

find . -name 'Cargo.toml' | sort -u | build_test_clippy