use combined_buffer::CombinedBuffer;
use std::mem;

pub struct Slicer<T> {
    slice_size: usize,
    buffer: Vec<T>,
}

impl<T> Slicer<T>
where
    T: Copy,
{
    pub fn new(slice_size: usize) -> Slicer<T> {
        Slicer {
            slice_size,
            buffer: Vec::new(),
        }
    }

    pub fn process<C: FnMut(Vec<T>) -> usize>(&mut self, data: &[T], mut consumer: C) {
        if self.buffer.len() + data.len() < self.slice_size {
            // Not enough data in buffer + data, collect into buffer
            self.buffer.extend_from_slice(data);
            return;
        }

        self.buffer = {
            let combined = CombinedBuffer::new(&self.buffer, data);
            let mut offset = 0;

            while offset + self.slice_size <= combined.len() {
                let slice = combined.read(offset, self.slice_size);
                let bytes_processed = consumer(slice);

                offset += bytes_processed;
            }

            let size = combined.len() - offset;
            let buffer = combined.read(offset, size);

            buffer
        }
    }

    pub fn flush(&mut self) -> Vec<T> {
        mem::replace(&mut self.buffer, Vec::new())
    }
}

pub struct FixedSlicer<T> {
    slicer: Slicer<T>,
    increment: usize,
}

impl<T> FixedSlicer<T>
where
    T: Copy,
{
    pub fn new(slice_size: usize, increment: usize) -> FixedSlicer<T> {
        FixedSlicer {
            slicer: Slicer::new(slice_size),
            increment,
        }
    }

    pub fn process<C: FnMut(Vec<T>)>(&mut self, data: &[T], mut consumer: C) {
        let increment = self.increment;
        self.slicer.process(data, |bytes| {
            consumer(bytes);

            increment
        });
    }
}

#[cfg(test)]
mod tests {
    use super::FixedSlicer;

    #[test]
    fn test_process() {
        let mut slicer = FixedSlicer::new(4, 2);

        let input = &[0i16, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        process_and_check(&mut slicer, &input[0..1], vec![]);
        process_and_check(&mut slicer, &input[1..3], vec![]);
        process_and_check(
            &mut slicer,
            &input[3..6],
            vec![vec![0, 1, 2, 3], vec![2, 3, 4, 5]],
        );
        process_and_check(&mut slicer, &input[6..9], vec![vec![4, 5, 6, 7]]);
        process_and_check(&mut slicer, &input[9..10], vec![vec![6, 7, 8, 9]]);
    }

    fn process_and_check(slicer: &mut FixedSlicer<i16>, data: &[i16], expected: Vec<Vec<i16>>) {
        let mut results = Vec::new();
        slicer.process(data, |v| results.push(v));
        assert_eq!(expected, results);
    }
}
