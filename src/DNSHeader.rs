use rand::Rng;

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
    fn new(
        &self,
        _qr:bool,
        _aa:bool,
        _tc:bool,
        _rd:u8,
        _ra:bool,
        _z:u8,
        _rcode:u16,
        _qdcount:u16,
        _ancount:u16,
        _nscount:u16,
        _arcount:u16
    ) -> Self {
        let mut rng = rand::thread_rng();

        DNSHeader{ 
            id: rng.gen(), 
            qr: _qr, 
            opcode: 0, 
            aa: _aa, 
            tc: _tc, 
            rd: _rd, 
            ra: _ra, 
            z: _z, 
            rcode: _rcode, 
            qdcount: _qdcount, 
            ancount: _ancount, 
            nscount: _nscount, 
            arcount: _arcount 
        }
    }

    fn getId(&self) -> u16 {
        self.id
    }
    fn getQr(&self) -> bool {
        self.qr
    }
    fn getOpcode(&self) -> u8 {
        self.opcode
    }
    fn getAa(&self) -> bool {
        self.aa
    }
    fn getTc(&self) -> bool {
        self.tc
    }
    fn getRd(&self) -> u8 {
        self.rd
    }
    fn getRa(&self) -> bool {
        self.ra
    }
    fn getZ(&self) -> u8 {
        self.z
    }
    fn getRcode(&self) -> u16 {
        self.rcode
    }
    fn getQdcount(&self) -> u16 {
        self.qdcount
    }
    
    fn getAncount(&self) -> u16 {
        self.ancount
    }
    
    fn getNscount(&self) -> u16 {
        self.nscount
    }
    
    fn getArcount(&self) -> u16 {
        self.arcount
    }

    fn setId(&mut self, _id:u16) {
        self.id=_id;
    }
    fn setQr(&mut self,_qr:bool) {
        self.qr=_qr;
    }
    fn setOpcode(&mut self,_opcode:u8) {
        self.opcode=_opcode;
    }
    fn setAa(&mut self,_aa:bool) {
        self.aa=_aa;
    }
    fn setTc(&mut self, _tc:bool) {
        self.tc=_tc;
    }
    fn setRd(&mut self, _rd:u8) {
        self.rd=_rd;
    }
    fn setRa(&mut self, _ra:bool) {
        self.ra=_ra;
    }
    fn setZ(&mut self, _z:u8) {
        self.z=_z;
    }
    fn setRcode(&mut self, _rcode:u16) {
        self.rcode=_rcode
    }
    fn setQdcount(&mut self,_qdcount:u16) {
        self.qdcount=_qdcount;
    }
    
    fn setAncount(&mut self,_ancount:u16) {
        self.ancount=_ancount;
    }
    
    fn setNscount(&mut self, _nscount:u16) {
        self.nscount=_nscount;
    }
    
    fn setArcount(&mut self, _arcount:u16) {
        self.arcount=_arcount;
    }

}