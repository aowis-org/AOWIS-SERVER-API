# default recipe, list all commands
default:
  @just --list

# --- Setup ---

# install necessary build tools
install-tools:
  cargo install cargo-watch
  cargo install cargo-cross

# run project setup
setup: install-tools
  cp -n .env.example .env || true

# --- Run and build ---

# build and run debug
run:
  cargo run

# watch mode (debug build); needs `cargo-watch`
watch:
  cargo watch -x 'run'

# build release
build:
  cargo build --release

# build debug
build-debug:
  cargo build

# --- Quality Control ---

# run all tests
test:
  cargo test

# lint with clippy
lint:
  cargo clippy

# --- Cleanup ---

# clean up build directories
clean:
  rm -rf target/
