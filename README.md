# A-Million-Digits-of-Pi

![rust v1.92](https://shields.io/badge/rust-1.92-blue?logo=rust)
![license](https://img.shields.io/github/license/tbali0524/a-million-digits-of-pi)

Calculate the digits of Pi with the Chudnovsky algorithm.

```txt
Usage:
    pi_digits.exe --generate         [from] [length]     generate digits of pi to file
    pi_digits.exe --encode                               encode pi digits to a compressed unicode string
    pi_digits.exe --hardcode                             generate hardcoded lookup values for pi slices
    pi_digits.exe --lookup-encoded   [from] [length]     lookup pi from encoded string
    pi_digits.exe --lookup-hardcoded [from] [length]     lookup pi from hardcoded slices
```

See also the [justfile](./justfile) for possible usage.
