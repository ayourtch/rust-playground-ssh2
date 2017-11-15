/* CLI connections */

mod traits;
mod ssh;
mod telnet;

use std;
use self::traits::DeviceInteraction;
use self::ssh::DeviceSSHConnection;
use self::telnet::DeviceTelnetConnection;

pub struct DeviceConnection {
    pub protocol: String,
    conn: Box<DeviceInteraction>,
}

impl DeviceConnection {
    pub fn new(proto: &str, targ: &str, c_user: &str, c_pass: &str) -> Self {
        // let mut ssh_conn = ssh::DeviceSSHConnection::new(targ, c_user, c_pass);
        let ccc: Box<DeviceInteraction> = match proto {
          "ssh" => Box::new(ssh::DeviceSSHConnection::new(targ, c_user, c_pass)),
          "telnet" => Box::new(telnet::DeviceTelnetConnection::new(targ, c_user, c_pass)),
          _ => panic!("Only SSH and Telnet are supported"),
        };

        let connection = DeviceConnection {
            protocol: proto.to_string(),
            conn: ccc, //Box::new(ssh_conn),
        };
        connection
    }

  pub fn write(&mut self, data: &str) -> std::io::Result<usize> {
    self.conn.write(data)
  }

  pub fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    self.conn.read(buf)
  }

  pub fn do_io(&mut self) {
    self.conn.do_io()
  }

  pub fn finish(&mut self) {
    self.conn.finish()
  }
}

