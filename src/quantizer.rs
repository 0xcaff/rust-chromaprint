pub struct Quantizer {
    t0: f64,
    t1: f64,
    t2: f64,
}

impl Quantizer {
    pub fn new(t0: f64, t1: f64, t2: f64) -> Quantizer {
        Quantizer { t0, t1, t2 }
    }

    /// Returns a value between 0 and 4 depending on where `value` falls in
    /// the range.
    pub fn quantize(&self, value: f64) -> u8 {
        if value < self.t1 {
            if value < self.t0 {
                0
            } else {
                1
            }
        } else {
            if value < self.t2 {
                2
            } else {
                3
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Quantizer;

    #[test]
    pub fn test_quantize() {
        let q = Quantizer::new(0.0, 0.1, 0.3);

        assert_eq!(0, q.quantize(-0.1));
        assert_eq!(1, q.quantize(0.0));
        assert_eq!(1, q.quantize(0.03));
        assert_eq!(2, q.quantize(0.1));
        assert_eq!(2, q.quantize(0.13));
        assert_eq!(3, q.quantize(0.3));
        assert_eq!(3, q.quantize(0.33));
        assert_eq!(3, q.quantize(1000.0));
    }
}
