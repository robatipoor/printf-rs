# prinf in rust

## add to Cargo.toml dependencies

```toml
[dependencies]
printf-rs = "0.1.0"
```

## Example

```rust
use printf_rs::*;

fn main() {
    printf!("%s \n", cstr!("Hello World !")); // print string
    printf!("%i \n", 1234); // print integer
}
```