use std::{io, time::Duration};

use tokio::{io::AsyncWriteExt, net::TcpStream, time::sleep};

macro_rules! with {
    ($selph:ident, $op:ident, $arg:expr) => {{
        if $selph.stream.is_none() {
            let stream = TcpStream::connect($selph.addr.as_str()).await?;
            $selph.stream = Some(stream);
        }
        let result = {
            let stream = $selph.stream.as_mut().unwrap();
            stream.$op($arg).await
        };
        if result.is_err() {
            $selph.stream = None;
        }
        result
    }};
}

struct Worker {
    addr: String,
    stream: Option<TcpStream>,
}

impl Worker {
    fn new(addr: &str) -> Self {
        Self {
            addr: addr.to_string(),
            stream: None,
        }
    }

    async fn send(&mut self, msg: &[u8]) -> io::Result<()> {
        with!(self, write_all, msg)
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut w = Worker::new("localhost:8000");
    loop {
        let result = w.send(b"foo\n").await;
        eprintln!("[debug] result: {:?}", result);
        sleep(Duration::from_secs(1)).await
    }
}
