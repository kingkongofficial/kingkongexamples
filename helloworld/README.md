## HelloWorld:

- helloworld
```rust
use kingkong::prelude::*;

routes! {
    GET "/" => |_| "Hello World";
}

fn main() {
    kingkong::run!();
}
```