pub enum DnsRType {
    A, 
    AAAA, 
    NS, 
    CNAME,
    PTR,
    MX
}

impl DnsRType {
    fn no(&self) -> i16{
        match self {
            A=> return 1,
            AAAA => return 28,
            NS=> return 2,
            CNAME=> return 5,
            PTR=> return 12,
            MX => return 15 
        }
    }
}