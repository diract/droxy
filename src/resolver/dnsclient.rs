use std::net::SocketAddr;
use std::cell::RefCell;
use std::fmt;

use futures::Future;
use tokio_core::reactor::Handle;
use trust_dns::error::*;
use trust_dns::op::Message;
use trust_dns::udp::UdpClientStream;
use trust_dns::tcp::TcpClientStream;
use trust_dns::client::ClientFuture;
use trust_dns::client::BasicClientHandle;
use trust_dns::client::ClientHandle;

pub struct DnsClient {
    fut_client: RefCell<BasicClientHandle>,
    server: SocketAddr,
    handle: Handle,
}

impl fmt::Display for DnsClient {
    fn fmt(&self, fmt: &mut fmt::Formatter)-> fmt::Result {
        write!(fmt, "{}", self.server)
    }
}

impl DnsClient {
    pub fn new(sa: SocketAddr, handle: Handle) -> Result<DnsClient, ClientError> {
        let (streamfut, streamhand) = UdpClientStream::new(sa, &handle.clone());
        let futclient = ClientFuture::new(streamfut, streamhand, &handle.clone(), None);

        Ok(DnsClient {
            fut_client: RefCell::new(futclient),
            server: sa,
            handle: handle,
        })
    }

    fn get_tcp_client(&self)-> BasicClientHandle {
        let (streamfut, streamhand) = TcpClientStream::new(self.server, &self.handle.clone());
        let futtcp = ClientFuture::new(streamfut, streamhand, &self.handle.clone(), None);
        futtcp
    }

    pub fn resolve(&self, q: Message, use_tcp: bool)-> Box<Future<Item=Message, Error=ClientError>> {
        let res = if use_tcp {
            let mut c = self.get_tcp_client();
            c.send(q)
        } else {
            self.fut_client.borrow_mut().send(q)
        };
        res
    }
}
