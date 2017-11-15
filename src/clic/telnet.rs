// extern crate ssh2;
use std;
// use std::io;
use super::traits::DeviceInteraction;

use std::net::{TcpStream};

// #[macro_use]
// extern crate rental;

/*
rental! {
  pub mod rentals {
    use super::*;

    #[rental_mut]
    pub struct DeviceTelnetConnection {
      session: Box<Session>,
      channel: Channel<'session>,
    }
  }
}
*/

pub struct DeviceTelnetConnection {
  tcp: TcpStream,
  // r: rentals::DeviceTelnetConnection,
}

impl DeviceTelnetConnection {
  pub fn new(targ: &str, c_user: &str, c_pass: &str) -> Self {
    use std::net::{TcpStream};
    let mut tcp = TcpStream::connect(targ).unwrap();

    let ret = DeviceTelnetConnection {
      tcp: tcp,
    };
    ret
  }
}

impl DeviceInteraction for DeviceTelnetConnection {
  fn write(&mut self, data: &str) -> std::io::Result<usize> {
    use std::io::prelude::*;
    println!("TELNET: writing: {}", data);
    self.tcp.write(data.as_bytes())
  }
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    // self.tcp.read(buf)
    println!("Read unimplemented!");
    Ok(0)
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
    println!("TCP closing");
  }
}

