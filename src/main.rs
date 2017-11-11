extern crate ssh2;

use ssh2::{Session, Channel, Error};
use std::net::{TcpStream};

struct DeviceSSHConnection <'h> {
  tcp: TcpStream,
  session: Session,
  target: String,
  username: String,
  password: Option<String>,
  // channel: Result<Channel <'h>, Error>, // Option<Channel<'h>>,
  channel_result: Result<Channel <'h>, Error>,
  channel: Option<Channel<'h>>,
}


impl <'h> DeviceSSHConnection <'h> {


  fn start(&mut self) -> () {
    println!("Starting!");
    self.session.handshake(&self.tcp).unwrap();
    self.session.set_timeout(5000);
    match self.password {
      Some(ref passwd) => { 
        self.session.userauth_password(&self.username, passwd).unwrap();
      },
      None => { panic!("Missing password!"); },
    };
    self.password = None;

  }

  fn get_channel <'x>(dev: &mut DeviceSSHConnection<'x>) {
    match (dev.session.channel_session()) {
      Ok(c) => { dev.channel = Some(c) },
      Err(e) => { dev.channel = None },
    }
  }

  fn start2(&mut self) {
    DeviceSSHConnection::get_channel(&mut self);

    match self.channel {
      Some(ref mut c) => {
        let res = c.shell().unwrap();
        println!("connection shell res = {:?}", res);
      },
      None => { panic!("No channel!"); },
    }
  }

  fn new(targ: &'h str, c_user: &str, c_pass: &str) -> Self {
    use std::net::{TcpStream};
    let tcp = TcpStream::connect(targ).unwrap();


    let mut connection = DeviceSSHConnection {
      tcp: tcp,
      target: targ.to_string(),
      username: c_user.to_string(),
      password: Some(c_pass.to_string()),
      session: Session::new().unwrap(),
      channel: None,
      channel_result: Err(Error::unknown()),
    };
    connection
  }
  
  fn log(&mut self, src: &str, lvl: i32, message: &str) {
  }

  fn send(&mut self, data: &str) -> std::io::Result<usize> {
    use std::io::prelude::*;
    match self.channel {
      Some(ref mut c) => {
        c.write(data.as_bytes())
      },
      None => { panic!("No channel!"); },
    }
  }
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    use std::io::prelude::*;
    match self.channel {
      Some(ref mut c) => {
        c.read(buf)
      },
      None => { panic!("No channel!"); },
    }
  }

  fn do_io(&mut self) {
    // use std::io::prelude::*;
    loop {
      let mut buf = [1u8; 16000];
      let res = self.read(&mut buf);
      println!("read res = {:?}", res);
      let s = String::from_utf8_lossy(&buf);
      println!("result: {}", s);
    };
  }

  fn finish(&mut self) {
    //use std::io::prelude::*;
    match self.channel {
      Some(ref mut c) => {
        c.wait_close();
        println!("{}", c.exit_status().unwrap());
      },
      None => { panic!("No channel!"); },
    }
  }

}


fn main() {
  println!("hello!");

  let target = "192.168.127.151:22";
  let mut conn = DeviceSSHConnection::new(&target, "user", "pass");
  conn.start();
  conn.start2();
  conn.send("term len 0\n");
  conn.send("show version\n");
  conn.do_io();
  conn.finish();
}
