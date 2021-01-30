use std::cell::RefCell;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::mem;

pub trait Input: io::Read {
    fn read_value(&mut self) -> io::Result<i32>;
}

pub trait Output: io::Write {
    fn write_value(&mut self, value: i32) -> io::Result<()>;
}

pub trait InputOutput: Input + Output {}

#[allow(dead_code)]
pub struct InputInterface<'stream> {
    stream: &'stream RefCell<Box<dyn InputOutput>>,
}

impl<'stream> Input for InputInterface<'stream> {
    fn read_value(&mut self) -> io::Result<i32> {
        self.stream.borrow_mut().read_value()
    }
}

impl<'stream> io::Read for InputInterface<'stream> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.borrow_mut().read(buf)
    }
}

#[allow(dead_code)]
impl<'stream> InputInterface<'stream> {
    pub fn new(stream: &'stream RefCell<Box<dyn InputOutput>>) -> InputInterface<'stream> {
        InputInterface { stream }
    }
}

#[allow(dead_code)]
pub struct OutputInterface<'stream> {
    stream: &'stream RefCell<Box<dyn InputOutput>>,
}

impl<'stream> Output for OutputInterface<'stream> {
    fn write_value(&mut self, value: i32) -> io::Result<()> {
        self.stream.borrow_mut().write_value(value)
    }
}

impl<'stream> io::Write for OutputInterface<'stream> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stream.borrow_mut().write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.stream.borrow_mut().flush()
    }
}

#[allow(dead_code)]
impl<'stream> OutputInterface<'stream> {
    pub fn new(stream: &'stream RefCell<Box<dyn InputOutput>>) -> OutputInterface<'stream> {
        OutputInterface { stream }
    }
}

#[allow(dead_code)]
pub struct MemoryStream {
    memory: Vec<i32>,
    buffer: Vec<u8>,
    cur_data: i32,
    position: i8,
}

impl Input for io::Stdin {
    fn read_value(&mut self) -> io::Result<i32> {
        Ok(self.bytes().next().unwrap()? as i32)
    }
}

impl Input for fs::File {
    fn read_value(&mut self) -> io::Result<i32> {
        let mut bytes: [u8; 4] = [0; 4];
        let mut iterator = self.bytes();
        for i in bytes.iter_mut() {
            *i = iterator.next().unwrap()?;
        }
        unsafe { Ok(mem::transmute::<[u8; 4], i32>(bytes)) }
    }
}

impl Input for MemoryStream {
    fn read_value(&mut self) -> io::Result<i32> {
        if !self.memory.is_empty() {
            Ok(self.memory.remove(0))
        } else {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF"))
        }
    }
}

impl Input for io::Empty {
    fn read_value(&mut self) -> io::Result<i32> {
        Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF"))
    }
}

impl Output for io::Stdout {
    fn write_value(&mut self, value: i32) -> io::Result<()> {
        // UTF-8
        self.write_all(&[value as u8])?;
        Ok(())
    }
}

impl Output for fs::File {
    fn write_value(&mut self, value: i32) -> io::Result<()> {
        // binary
        unsafe {
            let bytes = mem::transmute::<i32, [u8; 4]>(value);
            self.write_all(&bytes)?;
        }
        Ok(())
    }
}
impl Output for MemoryStream {
    fn write_value(&mut self, value: i32) -> io::Result<()> {
        self.memory.push(value);
        Ok(())
    }
}

impl Output for io::Sink {
    fn write_value(&mut self, _: i32) -> io::Result<()> {
        Ok(())
    }
}

impl InputOutput for MemoryStream {}

impl io::Read for MemoryStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if !self.memory.is_empty() || self.position > 0 {
            let mut size = 0;
            for field in buf {
                if self.position == 0 {
                    self.cur_data = self.memory.remove(0);
                }
                unsafe {
                    let bytes = mem::transmute::<i32, [u8; 4]>(self.cur_data);
                    *field = bytes[self.position as usize];
                }
                self.position += 1;
                size += 1;
                if self.position == 4 {
                    self.position = 0;
                    if self.memory.is_empty() {
                        return Ok(size);
                    }
                }
            }
            Ok(size)
        } else {
            Ok(0)
        }
    }
}

impl io::Write for MemoryStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut i = 0;
        for data in buf.iter() {
            self.buffer.push(*data);
            if self.buffer.len() == 4 {
                unsafe {
                    let mut data: [u8; 4] = [0; 4];
                    data.clone_from_slice(&self.buffer[0..4]);
                    let value = mem::transmute::<[u8; 4], i32>(data);
                    self.memory.push(value);
                }
                self.buffer.clear();
            }
            i += 1;
        }
        Ok(i)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[allow(dead_code)]
impl MemoryStream {
    pub fn new() -> MemoryStream {
        MemoryStream {
            memory: vec![],
            position: 0,
            buffer: vec![],
            cur_data: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::io::Read;
    use std::io::Write;

    #[test]
    fn write_and_read() {
        let mut stream = MemoryStream::new();
        stream.write_all(&[0, 1, 2, 3]).unwrap();
        for (i, byte) in stream.bytes().enumerate() {
            assert_eq!(i as u8, byte.unwrap());
        }
    }

    #[test]
    fn write_and_read_value() {
        let mut stream = MemoryStream::new();
        stream.write_value(42).unwrap();
        assert_eq!(stream.read_value().unwrap(), 42);
    }

    #[test]
    fn interfaces() {
        let stream: RefCell<Box<dyn InputOutput>> = RefCell::new(Box::new(MemoryStream::new()));
        let input = InputInterface::new(&stream);
        let mut output = OutputInterface::new(&stream);
        output.write_all(&[0, 1, 2, 3]).unwrap();
        for (i, byte) in input.bytes().enumerate() {
            assert_eq!(i as u8, byte.unwrap());
        }
    }

    #[test]
    fn partial_write() {
        let mut stream = MemoryStream::new();
        stream.write_all(&[0]).unwrap();
        stream.write_all(&[1, 2]).unwrap();
        stream.write_all(&[3, 4]).unwrap();
        stream.write_all(&[5, 6, 7]).unwrap();
        stream.write_all(&[8, 9, 10]).unwrap(); // stray values, shouldn't be read
        assert_eq!(stream.read_value().unwrap(), 0x03020100);
        assert_eq!(stream.read_value().unwrap(), 0x07060504);
        assert!(stream.read_value().is_err());
    }

    #[test]
    fn partial_read() {
        let mut stream = MemoryStream::new();
        stream.write_value(0x03020100).unwrap();
        stream.write_value(0x07060504).unwrap();
        let mut buf1 = [0u8];
        assert_eq!(stream.read(&mut buf1).unwrap(), 1);
        assert_eq!(buf1[0], 0);
        let mut buf2 = [0u8; 2];
        assert_eq!(stream.read(&mut buf2).unwrap(), 2);
        assert_eq!(buf2[0], 1);
        assert_eq!(buf2[1], 2);
        assert_eq!(stream.read(&mut buf2).unwrap(), 2);
        assert_eq!(buf2[0], 3);
        assert_eq!(buf2[1], 4);
        let mut buf4 = [0u8; 4];
        assert_eq!(stream.read(&mut buf4).unwrap(), 3);
        assert_eq!(buf4[0], 5);
        assert_eq!(buf4[1], 6);
        assert_eq!(buf4[2], 7);
        assert_eq!(buf4[3], 0);
    }
}
