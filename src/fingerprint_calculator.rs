use classifiers::Classifiers;
use filter::Filter;
use quantizer::Quantizer;
use rolling_integral_image::RollingIntegralImage;

const FILTER_WIDTH: usize = 16;

pub struct FingerprintCalculator {
    classifiers: Classifiers,
    image: RollingIntegralImage,
    fingerprint: Vec<u32>,
}

impl FingerprintCalculator {
    pub fn new(classifiers: Classifiers) -> FingerprintCalculator {
        FingerprintCalculator {
            classifiers,
            image: RollingIntegralImage::new(256),
            fingerprint: Vec::new(),
        }
    }

    fn calculate_subfingerprint(&self) -> u32 {
        let mut bits = 0u32;
        let offset = self.image.rows() - FILTER_WIDTH;

        for (filter, quantizer) in self.classifiers.iter() {
            let temp = gray_code(quantizer.quantize(filter.apply(&self.image, offset)));

            bits = (bits << 2) | (temp as u32);
        }

        bits
    }

    pub fn consume(&mut self, features: [f64; 12]) {
        self.image.add_row(features);

        if self.image.rows() >= FILTER_WIDTH {
            let subfingerprint = self.calculate_subfingerprint();
            self.fingerprint.push(subfingerprint);
        }
    }

    pub fn fingerprint(&self) -> &[u32] {
        &self.fingerprint
    }
}

fn gray_code(idx: u8) -> u8 {
    match idx {
        0 => 0,
        1 => 1,
        2 => 3,
        3 => 2,
        _ => 0,
    }
}
