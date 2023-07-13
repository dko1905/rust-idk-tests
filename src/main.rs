mod book;
use anyhow::{Ok, Result};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8081").await?;

    let mut i = 0;
    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let (mut reader, mut writer) = socket.split();

            println!("Thread({}) started", i);
            tokio::io::copy(&mut reader, &mut writer).await?;
            println!("Thread({}) stopped", i);
            Ok(())
        });
        i += 1;
    }
}
