mod signature;
mod crc;
mod read;
mod chunks;

pub enum State {
  INIT,
  IHDR,
  PLTE,// optional state
  IDAT,
  IEND,
  FIN,
}

pub struct Options {
  bit_depth: u8,
  colour_type: u8,
  height: u32,
  interlace: bool,
  plte: Vec<[u8; 3]>,
  state: State,
  width: u32,
  crc: crc::CRC
}

impl Options {
  pub fn new() -> Options {
    Options {
      bit_depth: 0,
      colour_type: 0,
      height: 0,
      interlace: false,
      plte: vec!(),
      state: State::INIT,
      width: 0,
      crc: crc::CRC::new(),
    }
  }
}