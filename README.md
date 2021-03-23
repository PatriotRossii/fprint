This crate entroduces `fprint!` macro which provides flushed print

```toml
[dependencies] 
fprint = "0.1"
```

## Usage

```rust
use std::io::Read;

fn main() 
    let mut buffer = String::new();
    fprint!("Enter your number: ");
    io::stdin().read_line(&mut buffer);
}
```
