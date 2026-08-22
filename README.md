# A-Million-Digits-of-Pi

![rust v1.98](https://shields.io/badge/rust-1.98-blue?logo=rust)
![build](https://img.shields.io/github/actions/workflow/status/tbali0524/a-million-digits-of-pi/qa.yml)
![license](https://img.shields.io/github/license/tbali0524/a-million-digits-of-pi)

Calculate the digits of Pi with the Chudnovsky algorithm.

```txt
Usage:
    pi_digits.exe --generate         [from] [length]  generate digits of Pi to file
    pi_digits.exe --encode                            encode Pi digits to a compressed unicode string
    pi_digits.exe --hardcode                          generate hardcoded lookup values for Pi slices
    pi_digits.exe --lookup-encoded   [from] [length]  lookup Pi from encoded string
    pi_digits.exe --lookup-hardcoded [from] [length]  lookup Pi from hardcoded slices

Constraints:
    from >= 0, length >= 1, from + length <= 1,000,000
```

See also the [justfile](./justfile) for possible usage.

## Build prerequisites (Linux)

```sh
# assuming Ubuntu 26.04
sudo apt install -y build-essential libgmp10 m4
```
