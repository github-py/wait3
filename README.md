# Wait3

![Build Status]
![Latest Version]
![crates.io]
![License]
![Commit Activity]
![Rust 1.70]

[Build Status]: https://github.com/github-py/wait3/actions/workflows/rust.yml/badge.svg
[Latest Version]: https://img.shields.io/crates/v/wait3.svg
[crates.io]: https://img.shields.io/crates/v/wait3.svg
[License]: https://img.shields.io/crates/l/wait3
[Commit Activity]: https://img.shields.io/github/last-commit/github-py/wait3
[Rust 1.70]: https://img.shields.io/badge/Rust%20Version-1.70-red


**Wait for arbitrary conditions (e.g.: time, TCP connetions, etc) in the command-line)**

---


## Installation


```bash
cargo install wait3
```

## Usage


### Wait for time

> number: a valid integer
> suffix: smh

```bash
wait3 time <number><suffix>
```

#### Examples

> Wait for 5 seconds
>
> ```bash
> wait3 time 5s
> ```

> Wait for 4 minutes
>
> ```bash
> wait3 time 4m
> ```

> Wait for subprocess
>
> ```bash
> wait3 command --exit 0 'sleep 9'
> ```
