use std::net::SocketAddr;
use std::io;
use std::error::Error;
use std::io::Read;
use std::fs;
use std::net;
use std::path;
use std::collections::BTreeMap;

use toml;

#[derive(Debug)]
pub struct RegionalResolvers {
    pub resolv: BTreeMap<String, SocketAddr>,
    pub default: SocketAddr,
}

#[derive(Deserialize, Debug)]
struct ConfValue {
    servers: BTreeMap<String, net::SocketAddr>,
    rule: BTreeMap<String, String>,
}

impl RegionalResolvers{
    pub fn new(conf: path::PathBuf) -> Result<RegionalResolvers, Box<Error>> {
        let f = fs::File::open(conf).unwrap();
        let mut bufreader = io::BufReader::new(f);
        let mut contents = String::new();
        bufreader.read_to_string(&mut contents).unwrap();

        let conf: ConfValue = toml::from_str(&contents)?;
        let servers = conf.servers;
        let default = conf.rule.get("else").and_then(|s| {
            servers.get(s)
        }).ok_or(io::Error::new(io::ErrorKind::NotFound, "no default dns server defined"))?;
        let mut resolv =  BTreeMap::new();
        for (region, server) in &conf.rule {
            let server_addr = servers.get(server).ok_or(
             io::Error::new(io::ErrorKind::NotFound, format!("dns server {} defined", server)))?;
            resolv.insert(region.clone(), server_addr.clone());
        }
        Ok(RegionalResolvers{
            resolv: resolv,
            default: default.clone(),
        })
    }
}
