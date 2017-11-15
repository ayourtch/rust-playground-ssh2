extern crate ssh2;

use ssh2::{Session, Channel, Error};
use std::net::{TcpStream};

#[macro_use]
extern crate rental;

rental! {
  pub mod rentals {
    use super::*;

    #[rental_mut]
    pub struct DeviceSSHConnection {
      session: Box<Session>,
      channel: Channel<'session>,
    }
  }
}

pub struct DeviceSSHConnection {
  tcp: TcpStream,
  r: rentals::DeviceSSHConnection,
}

impl DeviceSSHConnection {
  fn new(targ: &str, c_user: &str, c_pass: &str) -> Self {
    use std::net::{TcpStream};
    let mut session = Session::new().unwrap();
    let mut tcp = TcpStream::connect(targ).unwrap();

    session.handshake(&tcp).unwrap();
    session.set_timeout(5000);
    println!("Authenticating...");
    session.userauth_password(c_user, c_pass).unwrap();

    let mut sess = Box::new(session);
    let ret = DeviceSSHConnection {
      tcp: tcp,
      r: rentals::DeviceSSHConnection::new(
        sess,
        |s| {
          let mut chan = s.channel_session().unwrap();
          chan.shell().unwrap();
          chan
        }
      )
    };
    ret
  }
  fn write(&mut self, data: &str) -> std::io::Result<usize> {
    use std::io::prelude::*;
    self.r.rent_mut(|c| {
      c.write(data.as_bytes())
    })
  }
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    use std::io::prelude::*;
    self.r.rent_mut(|c| {
      c.read(buf)
    })
  }

  fn do_io(&mut self) {
    loop {
      let mut buf = [1u8; 16000];
      let res = self.read(&mut buf);
      println!("read res = {:?}", res);
      let s = String::from_utf8_lossy(&buf);
      println!("result: {}", s);
    };
  }

  fn finish(&mut self) {
    self.r.rent_mut(|c| {
      c.wait_close();
      println!("{}", c.exit_status().unwrap());
    });
  }
}



fn main() {
  use std::io::prelude::*;
  println!("hello!");
  let target = "192.168.127.151:22";

  let mut s2 = DeviceSSHConnection::new(&target, "aytest", "cisco123");
  s2.write("term len 0\n");
  s2.write("show version\n");
  s2.do_io();
}
