use std::io::{BufWriter, Write};
use std::net::TcpStream;

#[derive(Debug)]
struct MyWriter<W> {
    writer: W,
}

impl<W: Write> MyWriter<W> {
    pub fn new(addr: &str) -> MyWriter<BufWriter<TcpStream>> {
        let stream = TcpStream::connect(addr).unwrap();
        MyWriter {
            writer: BufWriter::new(stream),
        }
    }

    pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
        self.writer.write_all(buf.as_bytes())
    }
}

fn test() {
    let mut writer = MyWriter::<BufWriter<TcpStream>>::new("127.0.0.1:8888");
    writer.write("hello world!");
}