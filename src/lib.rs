extern crate bytes;
#[macro_use]
extern crate error_chain;
extern crate futures;
#[macro_use]
extern crate log;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_service;
extern crate trust_dns;
extern crate toml;
extern crate trust_dns_server;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use std::error::Error;
use std::str;
use std::sync::Arc;

mod resolver;
mod ruling;

pub fn run(config_path: &str)-> Result<(), Box<Error>> {
    let mut core = tokio_core::reactor::Core::new()?;

    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    let ruler = ruling::DomainMatcher::new(config_path)?;
    let d = Arc::new(ruler);

    let h = core.handle();
    let f = resolver::start_resolver(h, d.clone())?;
    core.run(f);
    Ok(())
}

