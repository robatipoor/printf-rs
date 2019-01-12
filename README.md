## prinf macro in rust
[![Crates.io](https://img.shields.io/crates/v/printf-rs.svg?style=plastic)](http://crates.io/crates/printf-rs)

### add to Cargo.toml dependencies

```toml
[dependencies]
printf-rs = "0.1.1"
libc = "0.2.46"
```

#### Example 1

```rust
use printf_rs::*;

fn main() {
    printf!("%s \n", cstr!("Hello World !")); // print string
    printf!("%i \n", 1234); // print integer
}
```
#### Example 2

```rust
use printf_rs::*;
use std::thread::sleep;
use std::time::Duration;
fn main() {
    let chars = vec!['a','b','c','d','e','f'];
    // print char with \r (carriage return) Moves the active position to the initial position of the current line.
    for c in chars {
        printf!("\r %lc ",c);
        sleep(Duration::from_secs(2));
    }
}
```

