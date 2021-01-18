use std::io::Read;

pub fn int_u_32(buf_reader: &mut Read) -> std::io::Result<u32> {
  let mut buffer = [0; 4];
  buf_reader.read(&mut buffer)?;
  Ok(u32::from_be_bytes(buffer))
}