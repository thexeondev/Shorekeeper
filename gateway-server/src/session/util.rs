pub struct LengthFieldBasedDecoder {
    buffer: Box<[u8]>,
    cur_index: usize,
}

impl LengthFieldBasedDecoder {
    const INITIAL_CAPACITY: usize = 8192;

    pub fn new() -> Self {
        Self {
            buffer: vec![0u8; Self::INITIAL_CAPACITY].into_boxed_slice(),
            cur_index: 0,
        }
    }

    pub fn input(&mut self, data: &[u8]) {
        self.ensure_capacity(data.len());

        (&mut self.buffer[self.cur_index..self.cur_index + data.len()]).copy_from_slice(data);
        self.cur_index += data.len();
    }

    pub fn pop_with<T>(&mut self, decode: impl FnOnce(&[u8]) -> Option<T>) -> Option<T> {
        if self.cur_index >= 3 {
            let frame_length = read_length_field(&self.buffer);
            let segment_size = 3 + frame_length;
            if self.cur_index >= segment_size {
                let retval = decode(&self.buffer[3..segment_size]);

                self.buffer.copy_within(segment_size..self.cur_index, 0);
                self.cur_index -= segment_size;

                return retval;
            }
        }

        None
    }

    #[inline]
    fn ensure_capacity(&mut self, add_len: usize) {
        if self.buffer.len() < self.cur_index + add_len {
            let mut new_buf = vec![0u8; self.cur_index + add_len];
            new_buf[..self.cur_index].copy_from_slice(&self.buffer[..self.cur_index]);
            self.buffer = new_buf.into_boxed_slice();
        }
    }
}

#[inline]
fn read_length_field(buf: &[u8]) -> usize {
    buf[0] as usize | (buf[1] as usize) << 8 | (buf[2] as usize) << 16
}
