run:
    cargo run

build:
    cargo build

release:
    cargo build --release

[linux]
install:
    cargo build --release
    cp ./target/release/Breadboard $HOME/.local/bin/breadboard
    cargo clean
