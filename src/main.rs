extern crate ssh2;

use ssh2::{Session, Channel, Error};
use std::net::{TcpStream};
extern crate owning_ref;

use owning_ref::OwningHandle;

struct DeviceSSHConnection {
  tcp: TcpStream,
  channel: OwningHandle<Box<Session>, Box<Channel<'static>>>,
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
    let mut oref = OwningHandle::new_with_fn(sess, unsafe { |x| Box::new((*x).channel_session().unwrap()) } );
    oref.shell().unwrap();
    let ret = DeviceSSHConnection {
      tcp: tcp,
      channel: oref
    };
    ret 
  }

  fn write(&mut self, data: &str) -> std::io::Result<usize> {
    use std::io::prelude::*;
    self.channel.write(data.as_bytes())
  }
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    use std::io::prelude::*;
    self.channel.read(buf)
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
    self.channel.wait_close();
    println!("{}", self.channel.exit_status().unwrap());
  }
}


fn conn() -> OwningHandle<Box<Session>, Box<Channel<'static>>> {
  let sess = Box::new(Session::new().unwrap());
  let oref = OwningHandle::new_with_fn(sess, unsafe { |x| Box::new((*x).channel_session().unwrap()) } );
  oref
}

fn main() {
  println!("hello!");
  let target = "192.168.127.151:22";

  let mut s2 = DeviceSSHConnection::new(&target, "aytest", "cisco123");
  s2.write("term len 0\n");
  s2.write("show version\n");
  s2.do_io();
}
