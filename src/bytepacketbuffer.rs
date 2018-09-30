use std::io::{Read,Error,ErrorKind};

pub struct BytePacketBuffer {
    pub buf: [u8;512],
    pub pos: usize
}

impl BytePacketBuffer {
    fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buf: [0,512],
            pos: 0
        }
    }

    fn pos(&self) -> usize {
        self.pos
    }

    fn step(&mut self,steps: usize) -> Result<()> {
        self.pos += steps;
        Ok(())
    }

    fn seek(&mut self,pos: usize) -> Result<()> {
        self.pos = pos;
        Ok(())
    }

    fn read(&mut self) -> Result<u8> {
        if self.pos >= 512 {
            return Err(Error::new(ErrorKind::InvalidInput, "End of buffer to read"));
        }
        let res = self.buf[self.pos];
        self.pos += 1;
        Ok(res)
    }

    fn get(&mut self) -> Result<u8> {
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

    fn read_u16(&mut self) -> Result<u16> {
        let res = ((self.read()? as u16) << 8) |
                    (self.read()? as u16);
        Ok(res)
    }

    fn read_u32(&mut self) -> Result<u32> {
        let res = ((self.read_u16()? as u32) << 16) |
                    (self.read_u16()? as u32);
        Ok(res)
    }

    fn read_qname(&mut self,outstr: &mut String) -> Result<()> {
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

}