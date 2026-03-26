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
    sed "s|Exec=breadboard|Exec=$HOME/.local/bin/breadboard|" breadboard.desktop > /tmp/breadboard.desktop
    desktop-file-install --dir=$HOME/.local/share/applications /tmp/breadboard.desktop
    cargo clean
