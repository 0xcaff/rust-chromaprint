const FILTER_COEFFICIENTS: [f64; 5] = [0.25, 0.75, 1.0, 0.75, 0.25];

pub struct ChromaFilter {
    filter_coefficients: &'static [f64],
    buffer: [[f64; 12]; 8],
    buffer_offset: usize,
    buffer_size: usize,
}

impl ChromaFilter {
    pub fn new(filter_coefficients: &'static [f64]) -> ChromaFilter {
        ChromaFilter {
            filter_coefficients,
            buffer: [[0f64; 12]; 8],
            buffer_offset: 0,
            buffer_size: 1,
        }
    }

    pub fn handle_features(&mut self, features: [f64; 12]) -> Option<[f64; 12]> {
        self.buffer[self.buffer_offset] = features;
        self.buffer_offset = (self.buffer_offset + 1) % 8;
        if self.buffer_size >= self.filter_coefficients.len() {
            let offset = (self.buffer_offset + 8 - self.filter_coefficients.len()) % 8;
            let mut result = [0.0f64; 12];

            for i in 0..12 {
                for j in 0..self.filter_coefficients.len() {
                    result[i] += self.buffer[(offset + j) % 8][i] * self.filter_coefficients[j];
                }
            }

            Some(result)
        } else {
            self.buffer_size += 1;
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ChromaFilter;

    #[test]
    fn blur2() {
        const COEFFICIENTS: [f64; 2] = [0.5, 0.5];
        let mut filter = ChromaFilter::new(&COEFFICIENTS);

        let d1 = [0.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let d2 = [1.0, 6.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let d3 = [2.0, 7.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

        assert_eq!(None, filter.handle_features(d1));
        let row1 = filter.handle_features(d2).unwrap();
        let row2 = filter.handle_features(d3).unwrap();

        assert_eq!(0.5, row1[0]);
        assert_eq!(1.5, row2[0]);
        assert_eq!(5.5, row1[1]);
        assert_eq!(6.5, row2[1]);
    }

    #[test]
    fn blur3() {
        const COEFFICIENTS: [f64; 3] = [0.5, 0.7, 0.5];
        let mut filter = ChromaFilter::new(&COEFFICIENTS);

        let d1 = [0.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let d2 = [1.0, 6.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let d3 = [2.0, 7.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let d4 = [3.0, 8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

        assert_eq!(None, filter.handle_features(d1));
        assert_eq!(None, filter.handle_features(d2));
        let row1 = filter.handle_features(d3).unwrap();
        let row2 = filter.handle_features(d4).unwrap();

        assert_ulps_eq!(1.7, row1[0]);
        assert_ulps_eq!(3.399999999999999, row2[0]);
        assert_ulps_eq!(10.199999999999999, row1[1]);
        assert_ulps_eq!(11.899999999999999, row2[1]);
    }

    #[test]
    fn diff() {
        const COEFFICIENTS: [f64; 2] = [1.0, -1.0];
        let mut filter = ChromaFilter::new(&COEFFICIENTS);

        let d1 = [0.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let d2 = [1.0, 6.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let d3 = [2.0, 7.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

        assert_eq!(None, filter.handle_features(d1));
        let row1 = filter.handle_features(d2).unwrap();
        let row2 = filter.handle_features(d3).unwrap();

        assert_eq!(-1.0, row1[0]);
        assert_eq!(-1.0, row2[0]);
        assert_eq!(-1.0, row1[1]);
        assert_eq!(-1.0, row2[1]);
    }
}
