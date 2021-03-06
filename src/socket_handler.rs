use constants::*;
use message::Message;
use endpoint::MsgHandler;

use mio::*;
use mio::udp::{UdpSocket};

pub struct SocketHandler<H>{
    sock: UdpSocket,
    handler: H
}

impl<H: MsgHandler>  SocketHandler<H> {
    pub fn new(sock: UdpSocket, handler: H) -> SocketHandler<H> {
        SocketHandler{
            sock: sock,
            handler: handler
        }
    }
}

impl<H: MsgHandler> Handler for SocketHandler<H> {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, _event_loop: &mut EventLoop<SocketHandler<H>>, token: Token, _: EventSet) {
        match token {
            SERVER => {
                let mut buf: [u8; 2048] = [0; 2048];
                let (len, addr) = self.sock.recv_from(&mut buf).unwrap().unwrap();

                let pkt = &buf[..len];

                match Message::from_bytes(pkt) {
                    Ok(msg) => {
                        match (self.handler).handle_msg(&addr, &msg) {
                            Some(resp) => {
                                self.sock.send_to(&resp, &addr).unwrap_or(None); // UDP is best-effort, right?
                            },
                            None => ()
                        }
                    },
                    Err(_) => ()
                }
            }
            _ => panic!("unexpected token"),
        }
    }

    fn notify(&mut self, _event_loop: &mut EventLoop<Self>, _msg: Self::Message) {
        println!("notify");
    }

    fn interrupted(&mut self, _event_loop: &mut EventLoop<Self>) {
        println!("interrupted");
    }
}
