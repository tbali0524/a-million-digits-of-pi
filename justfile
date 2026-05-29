# Config for [Just](https://just.systems/) command runner.

set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

# list all available recipes
default:
    @just --list

# check source (no change applied)
lint:
    cargo check
    cargo fmt --check
    cargo clippy

# open the generated docs
docs:
    cargo doc --no-deps --document-private-items --open

# run tests
test:
    cargo test

# run with all the features
run:
    cargo run --release
    cargo run --release -- --generate 0 1000000
    cargo run --release -- --encode
    cargo run --release -- --hardcode
    cargo run --release -- --lookup-encoded 0 10
    cargo run --release -- --lookup-hardcoded 0 10

# delete files generated during build & run
[unix]
clean:
    cargo clean
    [ -e result/pi.txt ]           && rm result/pi.txt
    [ -e result/decoded_295k.txt ] && rm result/decoded_295k.txt
    [ -e result/encoded_295k.txt ] && rm result/encoded_295k.txt
    [ -e result/hardcoded.txt ]    && rm result/hardcoded.txt

# delete files generated during build & run
[windows]
clean:
    cargo clean
    if (Test-Path "result/pi.txt")           { Remove-Item -Path "result/pi.txt" -Verbose }
    if (Test-Path "result/decoded_295k.txt") { Remove-Item -Path "result/decoded_295k.txt" -Verbose }
    if (Test-Path "result/encoded_295k.txt") { Remove-Item -Path "result/encoded_295k.txt" -Verbose }
    if (Test-Path "result/hardcoded.txt")    { Remove-Item -Path "result/hardcoded.txt" -Verbose }

# run all QA checks: lint, test, run, docs, clean
qa: lint test run docs clean
