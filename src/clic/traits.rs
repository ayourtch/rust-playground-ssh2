use std;

pub trait DeviceInteraction {

  fn write(&mut self, data: &str) -> std::io::Result<usize>;
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
  fn do_io(&mut self);
  fn finish(&mut self);
  
}

