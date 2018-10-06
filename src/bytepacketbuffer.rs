use std::io::{Read,Error,ErrorKind,Result};

pub struct BytePacketBuffer {
    pub buf: [u8;512],
    pub pos: usize
}

impl BytePacketBuffer {
    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buf: [0;512],
            pos: 0
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn step(&mut self,steps: usize) -> Result<()> {
        self.pos += steps;
        Ok(())
    }

    pub fn seek(&mut self,pos: usize) -> Result<()> {
        self.pos = pos;
        Ok(())
    }

    pub fn read(&mut self) -> Result<u8> {
        if self.pos >= 512 {
            return Err(Error::new(ErrorKind::InvalidInput, "End of buffer to read"));
        }
        let res = self.buf[self.pos];
        self.pos += 1;
        Ok(res)
    }

    pub fn write(&mut self,buf: u8) -> Result<()> {
        if self.pos >= 512 {
            return Err(Error::new(ErrorKind::InvalidInput,"End of buffer to write"));
        }
        self.buf[self.pos] = buf;
        self.pos += 1;
        Ok(())
    }

    fn get(&mut self,pos: usize) -> Result<u8> {
        if pos >= 512 {
            return Err(Error::new(ErrorKind::InvalidInput,"End of buffer to get"));
        }
        Ok(self.buf[pos])
    }

    fn get_range(&mut self,start: usize,len: usize) -> Result<&[u8]> {
        if start + len >= 512 {
            return Err(Error::new(ErrorKind::InvalidInput,"End of buffer to get range"))
        }
        Ok(&self.buf[start..start+len as usize])
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        let res = ((self.read()? as u16) << 8) |
                    (self.read()? as u16);
        Ok(res)
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        let res = ((self.read_u16()? as u32) << 16) |
                    (self.read_u16()? as u32);
        Ok(res)
    }

    pub fn read_qname(&mut self,outstr: &mut String) -> Result<()> {
        let mut pos = self.pos();
        let mut jumped = false;
        let mut delim = "";
        loop {
            let len = self.get(pos)?;
            if (len & 0xc0) == 0xc0 {
                if !jumped {
                    self.seek(pos+2)?;
                }
                let b2 = (self.get(pos+1)?) as u16;
                let offset = (((len as u16) ^ 0xc0) << 8) | b2;
                pos = offset as usize;
                jumped = true;
                continue;
            }
            pos += 1;
            if len == 0 {
                break;
            }
            outstr.push_str(delim);
            let str_buffer = self.get_range(pos, len as usize)?;
            outstr.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());
            delim = ".";
            pos += len as usize;
        }
        if !jumped {
            self.seek(pos)?;
        }
        Ok(())
    }

    pub fn write_u16(&mut self,buf: u16) -> Result<()> {
        if self.pos >= 512 {
            return Err(Error::new(ErrorKind::InvalidInput,"End of buffer to write_u16"))
        }
        self.write((buf << 8) as u8)?;
        self.write((buf & 0xff) as u8)?;
        Ok(())
    }

    pub fn write_u32(&mut self,buf: u32) -> Result<()> {
        if self.pos >= 512 {
            return Err(Error::new(ErrorKind::InvalidInput, "End of buffer to write_u32"))
        }
        self.write_u16((buf << 16) as u16)?;
        self.write_u16((buf & 0xffff)as u16)?;
        Ok(())
    }

    pub fn write_qname(&mut self,qname: &str) -> Result<()> {
        let split_str = qname.split('.').collect::<Vec<&str>>();
        for label in split_str {
            let len = label.len();
            self.write(len as u8)?;
            for b in label.as_bytes() {
                self.write(*b)?;
            }
        }
        self.write(0)?;
        Ok(())
    }

    pub fn set(&mut self,pos: usize,val: u8) -> Result<()> {
        self.buf[pos] = val;
        Ok(())
    }

    pub fn set_u16(&mut self,pos: usize,val: u16) -> Result<()> {
        self.buf[pos] = (val >> 8) as u8;
        self.buf[pos+1] = (val & 0xff) as u8;
        Ok(())
    }
}