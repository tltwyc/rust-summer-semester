# Assignment 5: Mini-Redis-Basic

用于 Rust 开发实训短学期课程

## Instructions

In the project directory, first

```
cargo build
```

and then

```
cargo run --bin server
```

Then, open another terminal, at the same path,

```
cargo run --bin client
```

## Notes

- Implemented commands: 

```
get zju
set zju 1897
del zju
ping
ping "hello"
```

- Any command with substring *"rust"* is filtered. 