use std::io::Read;
use super::chunks;
use super::read;
use super;

fn chunks(buf_reader: &mut Read) {
  let mut options = Options::new();
  // first chunk will be IHDR
  read_chunk(buf_reader, &mut options, chunks::ihdr::process);
  // return image details
  // Last chunk will be IEND
  // stop on fin
}

fn read_chunk(buf_reader: &mut Read, options: &mut Options, fn(&[u8], u32, &mut Options) -> Result((), ())) {
  // length
  let length = read::int_u_32(buf_reader);
  let mut buf[u8; length] = [0; length + 4];
  buf_reader.read(&mut buf);
  // crc
  let crc = read::int_u_32(buf_reader);
  let calc_crc = options.crc.run(buf, length);
  if crc != calc_crc {
    if (0b1000 & buf[1]) == 0 {
      return
    }
  }
  // type
  let (state) = process_chunk_type(buf[0..4], options.state);
  // data
  let data = buf[4..length];
}


fn process_chunk_type(buffer: &[u8; 4], options: &Options) -> (State, bool) {
  match options.state {
    State::INIT => init_chunk_type(&buffer),
    State::IHDR => ihdr_chunk_type(&buffer, options.colour_type == 3),
    State::PLTE => plte_chunk_type(&buffer),
    State::IDAT => idat_chunk_type(&buffer),
    State::IEND => iend_chunk_type(&buffer),
    _ => panic!("Unexpected state")
  }
}

fn init_chunk_type(buffer: &[u8; 4]) -> (State) {
  match buffer {
    b"IHDR" => (State::IHDR),
    chunk => {
      println!("Unexpected chunk in init: {}", chunk);
      (State::INIT)
    }
  }
}

fn ihdr_chunk_type(buffer: &[u8; 4], plte_required: bool) -> (State) {
  // plte is optional
  match buffer {
    b"PLTE" => (State::PLTE),
    b"IDAT" => {
      if plte_required {
        panic!("plte required. Found IDAT");
      }
      (State::IDAT)
    },
    b"cHRM" => (State::IHDR),
    b"gAMA" => (State::IHDR),
    b"iCCP" => (State::IHDR),
    b"sBIT" => (State::IHDR),
    b"sRGB" => (State::IHDR),
    b"pHYS" => (State::IHDR),
    b"sPLT" => (State::IHDR),
    b"tIME" => (State::IHDR),
    b"iTXt" => (State::IHDR),
    b"tEXt" => (State::IHDR),
    b"zTXt" => (State::IHDR),
    chunk => {
      println!("Unexpected chunk in ihdr: {}", chunk);
      (State::IHDR)
    }
  }
}

fn plte_chunk_type(buffer: &[u8; 4]) -> (State) {
  let mut buffer = [0; 4];
  buf_reader.read(&mut buffer)?;
  match buffer {
    b"IDAT" => (State::IDAT),
    b"bKGD" => (State::PLTE),
    b"hiST" => (State::PLTE),
    b"tRNS" => (State::PLTE),
    b"pHYS" => (State::PLTE),
    b"sPLT" => (State::PLTE),
    b"tIME" => (State::PLTE),
    b"iTXt" => (State::PLTE),
    b"tEXt" => (State::PLTE),
    b"zTXt" => (State::PLTE),
    chunk => {
      println!("Unexpected chunk in plte: {}", chunk);
      (State::PLTE)
    }
  }
}


fn idat_chunk_type(buffer: &[u8; 4]) -> (State) {
  let mut buffer = [0; 4];
  buf_reader.read(&mut buffer)?;
  match buffer {
    b"IDAT" => (State::IDAT),
    b"IEND" => (State::IEND),
    b"tIME" => (State::IEND),
    b"iTXt" => (State::IEND),
    b"tEXt" => (State::IEND),
    b"zTXt" => (State::IEND),
    chunk => {
      println!("Unexpected chunk in idat: {}", chunk);
      (State::IEND)
    }
  }
}

fn iend_chunk_type(buffer: &[u8; 4]) -> (State) {
  let mut buffer = [0; 4];
  buf_reader.read(&mut buffer)?;
  match buffer {
    b"IEND" => (State::FIN),
    b"tIME" => (State::IEND),
    b"iTXt" => (State::IEND),
    b"tEXt" => (State::IEND),
    b"zTXt" => (State::IEND),
    chunk => {
      println!("Unexpected chunk in iend: {}", chunk);
      (State::IEND)
    }
  }
}
