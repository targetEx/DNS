use crate::DnsRType::DnsRType;


pub struct DNSRR{
    name: Vec<u16>,
    rtype: DnsRType,
    class: u16,
    ttl : u32,
    rdlength : u16,
    rdata : Vec<u16>
}

impl DNSRR{
    fn new(
        _name:Vec<u16>,
        _rtype:DnsRType,
        _class:u16,
        _ttl:u32,
        _rdlength:u16,
        _rdata:Vec<u16>
    ) -> Self {
        DNSRR{
            name : _name,
            rtype : _rtype,
            class : _class,
            ttl : _ttl,
            rdlength : _rdlength,
            rdata : _rdata
        }
    }

    fn getName(&self) -> &Vec<u16> {
        &self.name
    }
    fn getRType(&self) -> &DnsRType { 
        &self.rtype
    }
    fn getClass(&self) -> u16 {
        self.class
    }
    fn getTtl(&self) -> u32 {
        self.ttl
    }
    fn getRdlength(&self) -> u16 {
        self.rdlength
    }
    fn getRdata(&self) -> &Vec<u16> {
        &self.rdata
    }

    fn setName(&mut self,_name:Vec<u16>) {
        self.name=_name;
    }
    fn setRType(&mut self, _rtype:DnsRType) { 
        self.rtype=_rtype
    }
    fn setClass(&mut self,_class:u16) {
        self.class=_class;
    }
    fn setTtl(&mut self, _ttl:u32) {
        self.ttl=_ttl;
    }
    fn setRdlength(&mut self,_rdlength:u16) {
        self.rdlength=_rdlength;
    }
    fn setRdata(&mut self,_rdata:Vec<u16>) {
        self.rdata=_rdata;
    }
}