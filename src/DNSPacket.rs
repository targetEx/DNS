use crate::DNSHeader::DNSHeader;
pub struct DNSPAcket{
    header : DNSHeader
}

impl DNSPAcket{
    fn new(_header:DNSHeader) -> Self {
        DNSPAcket { 
            header:_header 
        }
    }

    fn getHeader(&self) -> &DNSHeader {
        &self.header
    }

    fn setHeader(&mut self, _header:DNSHeader) {
        self.header=_header;
    }   
}