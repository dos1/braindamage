use std::io;
use std::io::Read;
use std::io::Write;
use std::fs;
use std::mem;

pub trait Input: io::Read {
    fn read_value(&mut self) -> io::Result<i32>;
}

pub trait Output: io::Write {
    fn write_value(&mut self, value: i32) -> io::Result<()>;
}

pub trait Stream: Input + Output {}

#[allow(dead_code)]
pub struct MemoryStream {
    memory: Vec<i32>,
    buffer: Vec<u8>,
    position: i8
}

impl Input for io::Stdin {
    fn read_value(&mut self) -> io::Result<i32> {
        return Ok(try!(self.bytes().nth(0).unwrap()) as i32);
    }
}

impl Input for fs::File {
    fn read_value(&mut self) -> io::Result<i32> {
        let mut bytes:[u8; 4] = [0; 4];
        let mut iterator = self.bytes();
        for i in 0..3 {
            bytes[i] = try!(iterator.next().unwrap());
        }
        unsafe {
            return Ok(mem::transmute::<[u8; 4], i32>(bytes));
        }
    }
}

impl Input for MemoryStream {
    fn read_value(&mut self) -> io::Result<i32> {
        if self.memory.len() > 0 {
            Ok(self.memory.remove(0))
        } else {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF"))
        }
    }
}

impl Output for io::Stdout {
    fn write_value(&mut self, value: i32) -> io::Result<()> {
        // UTF-8
        try!(self.write(&[value as u8]));
        Ok({})
    }
}

impl Output for fs::File {
    fn write_value(&mut self, value: i32) -> io::Result<()> {    
        // binary
        unsafe {
            let bytes = mem::transmute::<i32, [u8; 4]>(value);
            try!(self.write(&bytes));
        }
        Ok({})
    }
}
impl Output for MemoryStream {
    fn write_value(&mut self, value: i32) -> io::Result<()> {
        self.memory.push(value);
        Ok({})
    }
}

impl io::Read for MemoryStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.memory.len() > 0 {
            let mut data = self.memory.remove(0);
            let mut size = 0;
            for field in buf {
                unsafe {
                    let bytes = mem::transmute::<i32, [u8; 4]>(data);
                    *field = bytes[self.position as usize];
                }
                self.position += 1;
                size += 1;
                if self.position == 4 {
                    self.position = 0;
                    if self.memory.len() > 0 {
                        data = self.memory.remove(0);
                    } else {
                        return Ok(size)
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
    
    fn flush(&mut self) -> io::Result<()> { Ok({}) }
}

#[allow(dead_code)]
impl MemoryStream {
    pub fn new() -> MemoryStream {
        MemoryStream { memory: vec![], position: 0, buffer: vec![] }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use std::io::Write;

    #[test]
    fn write_and_read() {
        let mut stream = MemoryStream::new();
        stream.write(&[0, 1, 2, 3]).unwrap();
        for (i, byte) in stream.bytes().enumerate() {
            assert!(i as u8 == byte.unwrap());
        }
    }
}
