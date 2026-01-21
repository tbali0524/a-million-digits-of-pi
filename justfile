# Config file for [Just](https://just.systems/)

set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

lint:
    cargo check
    cargo fmt --check
    cargo clippy

docs:
    cargo doc --no-deps --document-private-items --open

test:
    cargo test

run:
    cargo run --release
    cargo run --release -- --generate 0 1000000
    cargo run --release -- --encode
    cargo run --release -- --hardcode
    cargo run --release -- --lookup-encoded 0 10
    cargo run --release -- --lookup-hardcoded 0 10

clean:
    cargo clean
    del result/pi.txt
    del result/decoded_295k.txt
    del result/encoded_295k.txt
    del result/hardcoded.txt
