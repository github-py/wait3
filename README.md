# Wait3

![Build Status]([Build Status])
![Actions]([Actions])
![Latest Version]([Latest Version])

[crates.io]([crates.io])
[[Rust 1.70]]

[Build Status]: https://img.shields.io/github/actions/workflow/status/poemscodes/wait3/ci.yml?branch=doctor
[Actions]: https://github.com/poemscodes/wait3/actions?query=branch%3Adoctor
[Latest Version]: https://img.shields.io/crates/v/wait3.svg
[crates.io]: https://crates.io/crates/wait3
[Rust 1.70]: https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html

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
