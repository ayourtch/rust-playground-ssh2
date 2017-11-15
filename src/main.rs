extern crate ssh2;
#[macro_use]
extern crate rental;
mod clic;



fn main() {
//  use std::io::prelude::*;
  println!("hello!");
  let target = "192.168.127.151:22";
  let proto = "ssh";
  let proto = "telnet";

  let mut s2 = clic::DeviceConnection::new(proto, &target, "aytest", "cisco123");
  s2.write("term len 0\n");
/*
  s2.write("show version\n");
  s2.do_io();
*/
}
