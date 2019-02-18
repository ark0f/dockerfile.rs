# dockerfile-rs
[![Build Status](https://travis-ci.org/ark0f/dockerfile.rs.svg?branch=master)](https://travis-ci.org/ark0f/dockerfile.rs)
![License](https://img.shields.io/crates/l/dockerfile_rs.svg)
[![crates.io](https://img.shields.io/crates/v/dockerfile-rs.svg)](https://crates.io/crates/dockerfile-rs)
[![API docs](https://docs.rs/dockerfile-rs/badge.svg)](https://docs.rs/dockerfile-rs)
[![Codecov](https://codecov.io/gh/ark0f/dockerfile.rs/branch/master/graph/badge.svg)](https://codecov.io/gh/ark0f/dockerfile.rs)

Correct `Dockerfile` generator library

# Quick start
```rust
use std::{io::{Result, Write}, fs::File};
use dockerfile_rs::{DockerFile, Copy, FROM};

fn main() -> Result<()> {
    let file = DockerFile::from(FROM!(nginx:latest))
        .comment("open port for server")
        .expose(80)
        .copy(Copy {
            src: ".".to_string(),
            dst: ".".to_string(),
            from: None,
            chown: None,
        })
        .cmd(&["echo", "Hello from container!"]);

    // generate Dockerfile into string
    let content = file.to_string();
    // write into file
    let mut file = File::create("nginx.Dockerfile")?;
    write!(&mut file, "{}", content)?;
    
    Ok(())
}
```

Generated file:
```Dockerfile
FROM nginx:latest

# open port for server
EXPOSE 80
COPY . .

CMD ["echo", "Hello from container!"]
```

# [Changelog](https://github.com/ark0f/dockerfile.rs/blob/master/CHANGELOG.md)

# License
dockerfile-rs under either of:

* [Apache License 2.0](https://github.com/ark0f/dockerfile.rs/blob/master/LICENSE-APACHE.md)
* [MIT](https://github.com/ark0f/dockerfile.rs/blob/master/LICENSE-MIT.md)

at your option.
