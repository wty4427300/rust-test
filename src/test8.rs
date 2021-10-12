use std::fmt;
use std::io::Write;

struct BufBuilder{
    buf: Vec<u8>,
}

//构造函数
impl BufBuilder{
    pub fn new() -> Self{
        Self{
            buf:Vec::with_capacity(1024),
        }
    }
}

//实现debug trait,打印字符串
impl fmt::Debug for BufBuilder {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,"{}",String::from_utf8_lossy(&self.buf))
    }
}

impl Write for BufBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
       //把buf添加到BufBuilder的尾部
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        //内存操作不需要落盘
        Ok(())
    }
}

fn test(){
    let mut buf=BufBuilder::new();
    buf.write_all(b"hello world").unwrap();
    println!("{:?}",buf)
}