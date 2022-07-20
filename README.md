# win-screenshot
Tiny wrapper around tokio mpsc that provides bidirectional channel

## Examples
```rust
use tokio_bidichannel::unbounded_bidichannel;

#[tokio::main]
async fn main() {
    let (mut left, mut right) = unbounded_bidichannel::<&str, &str>();

    tokio::spawn(async move {
        loop {
            left.send("hello, what time is it now?").unwrap();
            dbg!(left.recv().await);
        }
    });

    tokio::spawn(async move {
        loop {
            dbg!(right.recv().await);
            right.send("time to drink a cup of tea").unwrap();
        }
    }).await;
}
```