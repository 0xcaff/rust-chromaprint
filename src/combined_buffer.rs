pub struct CombinedBuffer<'a, 'b, T> where T: 'a + 'b {
    a: &'a [T],
    b: &'b [T],
}

impl <'a, 'b, T> CombinedBuffer<'a, 'b, T> where T: Clone {
    pub fn new(a: &'a [T], b: &'b [T]) -> CombinedBuffer<'a, 'b, T> {
        CombinedBuffer {
            a,
            b,
        }
    }

    pub fn len(&self) -> usize {
        self.a.len() + self.b.len()
    }

    pub fn read(&self, start_idx: usize, size: usize) -> Vec<T> {
        if start_idx + size < self.a.len() {
            return self.a[start_idx..(start_idx + size)].to_vec()
        }

        if start_idx >= self.a.len() {
            let b_start_idx = start_idx - self.a.len();
            return self.b[b_start_idx..(b_start_idx + size)].to_vec();
        }

        let mut from_a = self.a[start_idx..].to_vec();
        let remaining_len = size - from_a.len();
        from_a.extend_from_slice(&self.b[..remaining_len]);

        from_a
    }
}
