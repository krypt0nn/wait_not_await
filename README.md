<h1 align="center">🦀 wait_not_await</h1>

Simple awaiter implementation in Rust

## Examples

### Await as a variable

```rs
use std::time::Duration;

use wait_not_await::Await;

let mut awaiter = Await::new(move || {
    std::thread::sleep(Duration::from_secs(3));

    "Hello, Wolrd!".to_string()
});

if let Some(result) = awaiter.wait(None) {
    println!("Result: {}", result);
}
```

### Await with functions

```rs
use std::time::Duration;

use wait_not_await::Await;

fn async_hello_world() -> Await<String> {
    Await::new(move || {
        std::thread::sleep(Duration::from_secs(2));

        "Hello, World!".to_string()
    })
}

println!("{}", async_hello_world().wait(None).unwrap());
```

### Await loop with result

```rs
use std::time::Duration;

use wait_not_await::Await;

fn async_hello_world() -> Await<String> {
    Await::new(move || {
        std::thread::sleep(Duration::from_secs(2));

        "Hello, World!".to_string()
    })
}

let mut awaiter = async_hello_world();
let mut i = 1;

while let None = awaiter.result() {
    println!("Waiting for result: {}", i);

    i += 1;
}

println!("{}", awaiter.result().unwrap());
```

Author: [Nikita Podvirnyy](https://vk.com/technomindlp)

Licensed under [MIT](LICENSE)
