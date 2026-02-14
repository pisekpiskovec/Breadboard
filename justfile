run:
    cargo run

build:
    cargo build

release:
    cargo build --release

install:
    cargo build --release
    cp ./target/release/Breadboard $HOME/.local/bin/breadboard
    cargo clean
