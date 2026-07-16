# --- Setup ---

install-tools:
  cargo install cargo-watch

setup: install-tools
  cp -n .env.example .env || true


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

test:
  cargo test

clean:
  rm -rf target/
