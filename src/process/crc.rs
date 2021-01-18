

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    let crc = CRC::new();
  }

  #[test]
  fn test_crc_zero() {
    let crc = CRC::new();
    assert_eq!(crc.run(&[0], 1), 3523407757);
  }

  #[test]
  fn test_crc() {
    let crc = CRC::new();
    assert_eq!(crc.run(&[0x49, 0x48, 0x44, 0x52, 0, 0, 0, 0x20, 0, 0, 0, 0x20, 0x10, 0x06, 0, 0, 0], 17), 0x23EAA6B7);
  }
  #[test]
  fn test_incorrect_crc() {
    let crc = CRC::new();
    // error at pos 4 (0x01)
    let data = [0x49, 0x48, 0x44, 0x52, 0x01, 0, 0, 0x20, 0, 0, 0, 0x20, 0x10, 0x06, 0, 0, 0];
    let crc_s = crc.run(&data, 17);
    let crc_r = 0x23EAA6B7;
    assert_ne!(crc_s, crc_r);
  }
}

pub struct CRC {
  /// table of all messages
  table: [u32; 256],
}
impl CRC {
  /// populate table
  pub fn new() -> CRC {
    let mut table: [u32; 256] = [0; 256];
    for n in 0..256 {
      table[n as usize] = (0..8).fold(n as u32, |acc, _| {
        match acc & 1 {
          1 => 0xedb88320 ^ (acc >> 1),
          _ => acc >> 1,
        }
      });
    }
    CRC {
      table: table,
    }
  }
  /// Update a running CRC with the bytes buf[0..len-1]--the CRC
  /// should be initialized to all 1's, and the transmitted value
  /// is the 1's complement of the final running CRC (see the
  /// crc() routine below).
  pub fn update(&self, crc: u32, buf: &[u8], len: usize) -> u32 {
    let mut c = crc;
    for n in 0..len {
      c = (c >> 8) ^ self.table[((c & 0xff) ^ buf[n] as u32) as usize];
    }
    !c
  }

  /// Return the CRC of the bytes buf[0..len-1].
  pub fn run(&self, buf: &[u8], len: usize) -> u32 {
    self.update(!0, buf, len)
  }
}