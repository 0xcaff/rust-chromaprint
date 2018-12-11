pub struct BitWriter<'a> {
    /// Output to which stuff will be written.
    output: &'a mut [u8],

    /// Index at which the next byte will be written to in `output`.
    output_index: usize,

    /// A staging area for bits to be written to before they are written to `output`.
    buffer: u16,

    /// A number between 0 and 16 indicating how many bits of `buffer` are full.
    buffer_size: u8,
}

impl<'a> BitWriter<'a> {
    pub fn new(output: &'a mut [u8]) -> BitWriter<'a> {
        BitWriter {
            output,
            output_index: 0,
            buffer: 0,
            buffer_size: 0,
        }
    }

    pub fn write_all(&mut self, values: &[u8], bits: u8) {
        values
            .into_iter()
            .for_each(|value| self.write(*value, bits));
    }

    /// Writes a `bits` number of bits from `value` to the stream.
    pub fn write(&mut self, value: u8, bits: u8) {
        self.buffer |= (value as u16) << self.buffer_size;
        self.buffer_size += bits;

        while self.buffer_size >= 8 {
            self.write_buffer_to_output();
        }
    }

    /// Writes the buffered bits into the output. Returns the amount of space used in the output.
    pub fn flush(&mut self) -> usize {
        while self.buffer_size > 0 {
            self.write_buffer_to_output();
        }

        self.buffer_size = 0;
        self.output_index
    }

    fn write_buffer_to_output(&mut self) {
        self.output[self.output_index] = (self.buffer & 255 as u16) as u8;
        self.output_index += 1;
        self.buffer >>= 8;
        self.buffer_size = u8::saturating_sub(self.buffer_size, 8);
    }

    pub fn write_all_into(input: &[u8], bits: u8, output: &mut [u8]) -> usize {
        let mut writer = BitWriter::new(output);
        writer.write_all(input, bits);
        writer.flush()
    }
}
