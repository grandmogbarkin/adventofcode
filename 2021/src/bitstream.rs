#[derive(Debug)]
pub struct BitStream {
    cur_byte: usize,
    cur_bit: u8,
    raw_bytes: Vec<u8>,
}

impl Default for BitStream {
    fn default() -> Self {
        BitStream::new(&[])
    }
}

impl BitStream {
    pub fn new(bytes: &[u8]) -> Self {
        BitStream {
            cur_byte: 0,
            cur_bit: 1 << 7,
            raw_bytes: bytes.to_owned(),
        }
    }

    pub fn read_bits(&mut self, len: usize) -> u16 {
        self.by_ref()
            .take(len)
            .fold(0, |res, b| (res << 1) + b)
    }
}

impl Iterator for BitStream {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_byte >= self.raw_bytes.len() {
            return None;
        }

        let bit = if (self.raw_bytes[self.cur_byte] & self.cur_bit) > 0 {
            1
        } else {
            0
        };
        self.cur_bit >>= 1;
        if 0 == self.cur_bit {
            self.cur_bit = 1 << 7;
            self.cur_byte += 1;
        }

        Some(bit)
    }
}
