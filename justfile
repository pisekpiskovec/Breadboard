run:
    cargo run

build:
    cargo build

test:
    cargo test

release:
    cargo build --release

[linux]
install:
    cargo build --release
    cp ./target/release/Breadboard $HOME/.local/bin/breadboard
    desktop-file-install --dir=$HOME/.local/share/applications breadboard.desktop
    update-desktop-database ~/.local/share/applications
    cargo clean
