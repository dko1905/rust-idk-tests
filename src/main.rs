mod book;
use anyhow::{bail, Result};
use serde::Deserialize;
use std::str;
use thiserror::Error;
use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8081").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let (raw_reader, raw_writer) = socket.split();
            let mut reader = BufReader::new(raw_reader);
            let mut writer = BufWriter::new(raw_writer);

            match handler(&mut reader, &mut writer).await {
                Ok(_) => {}
                Err(e) => {
                    _ = writer
                        .write_all(format!("Error: {:?}\n", e).as_bytes())
                        .await;
                    _ = writer.flush().await;
                    ()
                }
            }
        });
    }
}

async fn handler(
    reader: &mut BufReader<ReadHalf<'_>>,
    writer: &mut BufWriter<WriteHalf<'_>>,
) -> Result<(), CommunicationError> {
    loop {
        writer.write_all(b"#>").await?;
        writer.flush().await?;
        let mut cmd_buf = String::with_capacity(5);
        reader.read_line(&mut cmd_buf).await?;

        let cmd = Cmd::from_str(&cmd_buf.trim_end())?;
        match cmd {
            Cmd::GET => writer.write_all(b"GET\n").await?,
            Cmd::PUT => writer.write_all(b"PUT\n").await?,
        };
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
enum Cmd {
    GET,
    PUT,
}

impl Cmd {
    fn from_str(s: &str) -> Result<Cmd> {
        match s {
            "get" | "GET" => Ok(Cmd::GET),
            "put" | "PUT" => Ok(Cmd::PUT),
            _ => bail!(CommunicationError::InvalidCmd(s.to_owned())),
        }
    }
}

#[derive(Error, Debug)]
enum CommunicationError {
    #[error("invalid command \"{0}\"")]
    InvalidCmd(String),
    #[error("io error")]
    IoError(#[from] io::Error),
    #[error("unknown error")]
    Unknown(#[from] anyhow::Error),
}
