# Project 1: Myfind

用于 Rust 开放实训短学期课程

## Instructions

- In the project directory:

```
cargo build
```

- Run. Argument format:

```
./target/debug/myfind <search_path> <regex> <optional: -d/-f>
```

- `-d`: only search for directories
- `-f`: only search for files

For example:
```
./target/debug/myfind . [abc] -d
``` 

- You can also build it in release mode, which takes longer to finish.
