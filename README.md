# kRedis

This crate aims to provide a clean and simple client library for Redis that works well in asynchronous Rust applciations.

This crate is still very new, and currently only provides very simple commands

## Example

```rust
use anyhow::Result;
use redis::Redis;

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut redis = Redis::connect("tcp://localhost:6379").await?;

    redis.ping().await?;
    redis.set(b"test", b"weee").await?;
    let x: String = redis.get(b"test").await?.try_into()?;
    println!("x: {}", x);

    Ok(())
}
```
