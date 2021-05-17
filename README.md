# doric
A highly simple asynchronous logging lib for Rust

# Quick Start
To use `doric`, add the dependencies to your `Cargo.toml`
```toml
[dependencies]
doric = { git = 'https://github.com/inkhare/doric', branch = 'main' }
```

Testing:
```
    cargo run --example test
```
    
## Example ##

```Rust
use doric::config;
use log::{info, error};
use std::{thread, time};

fn simple_log() {
    let conf = config::Config {
        path: "./log".to_string(),
        max_size: 2,
        max_segments: 3,
        interval: 10,
        level: doric::Info,
        log_type: doric::File,
    };

    config::init_config(&conf);
    let delay = time::Duration::from_millis(10);
    thread::sleep(delay);

    error!("error log test{:?}", 9090);
}

```

## NOTE

This library is verified to work in rustc 1.51.0 (nightly), and the support 
of other versions needs more testing.
