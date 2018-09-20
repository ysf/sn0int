use errors::*;
use json::LuaJsonValue;
use std::fmt;
use serde_json;
use schema::*;
// use std::convert::AsRef;


#[derive(Debug, Serialize, Deserialize)]
pub enum Object {
    Subdomain(NewSubdomainOwned),
    IpAddr(NewIpAddrOwned),
    SubdomainIpAddr(NewSubdomainIpAddr),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Subdomain(x) => write!(f, "Subdomain: {:?}", x.value),
            Object::IpAddr(x) => write!(f, "IpAddr: {:?}", x.value),
            Object::SubdomainIpAddr(x) => write!(f, "Subdomain->IpAddr: {}->{}", x.subdomain_id, x.ip_addr_id),
        }
    }
}

#[derive(Identifiable, Queryable, Serialize, PartialEq, Debug)]
#[table_name="domains"]
pub struct Domain {
    pub id: i32,
    pub value: String,
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Insertable)]
#[table_name="domains"]
pub struct NewDomain<'a> {
    pub value: &'a str,
}

#[derive(Identifiable, Queryable, Associations, Serialize, PartialEq, Debug)]
#[belongs_to(Domain)]
#[table_name="subdomains"]
pub struct Subdomain {
    pub id: i32,
    pub domain_id: i32,
    pub value: String,
}

impl fmt::Display for Subdomain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Insertable)]
#[table_name="subdomains"]
pub struct NewSubdomain<'a> {
    pub domain_id: i32,
    pub value: &'a str,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name="subdomains"]
pub struct NewSubdomainOwned {
    pub domain_id: i32,
    pub value: String,
}

impl NewSubdomainOwned {
    pub fn from_lua(x: LuaJsonValue) -> Result<NewSubdomainOwned> {
        let x = serde_json::from_value(x.into())?;
        Ok(x)
    }
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[table_name="ipaddrs"]
pub struct IpAddr {
    pub id: i32,
    pub family: String,
    pub value: String,
}

#[derive(Insertable)]
#[table_name="ipaddrs"]
pub struct NewIpAddr<'a> {
    pub family: &'a str,
    pub value: &'a str,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name="ipaddrs"]
pub struct NewIpAddrOwned {
    pub family: String,
    pub value: String,
}

impl NewIpAddrOwned {
    pub fn from_lua(x: LuaJsonValue) -> Result<NewIpAddrOwned> {
        let x = serde_json::from_value(x.into())?;
        Ok(x)
    }
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Subdomain)]
#[belongs_to(IpAddr)]
#[table_name="subdomain_ipaddrs"]
pub struct SubdomainIpAddr {
    pub id: i32,
    pub subdomain_id: i32,
    pub ip_addr_id: i32,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name="subdomain_ipaddrs"]
pub struct NewSubdomainIpAddr {
    pub subdomain_id: i32,
    pub ip_addr_id: i32,
}

impl NewSubdomainIpAddr {
    pub fn from_lua(x: LuaJsonValue) -> Result<NewSubdomainIpAddr> {
        let x = serde_json::from_value(x.into())?;
        Ok(x)
    }
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Subdomain)]
#[table_name="urls"]
pub struct Url {
    pub id: i32,
    pub subdomain_id: i32,
    pub status: u16,
    pub body: Vec<u8>,
}
