pub struct DNSHeader{
    id : u16,
    qr : bool,
    opcode : u8,
    aa : bool,
    tc : bool,
    rd : u8,
    ra : bool,
    z : u8,
    rcode : u16,
    qdcount: u16,
    ancount : u16,
    nscount : u16,
    arcount : u16
}

impl DNSHeader{
    fn new() -> Self {
        return DNSHeader{};
    }
}