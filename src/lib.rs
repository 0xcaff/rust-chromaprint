extern crate rustfft;

mod chroma;
mod chroma_filter;
mod chroma_normalize;
mod combined_buffer;
mod fft;
mod filter;
mod fingerprint_calculator;
mod quantizer;
mod resampler;
mod rolling_integral_image;
mod slicer;

#[cfg(test)]
#[macro_use]
extern crate approx;

#[cfg(test)]
mod tests;

pub struct Fingerprinter;

impl Fingerprinter {
    pub fn new() -> Fingerprinter {
        Fingerprinter
    }

    pub fn feed(&mut self, raw_pcm: &[i16]) {
        unimplemented!()
    }

    pub fn fingerprint(self) -> Fingerprint {
        unimplemented!()
    }
}

pub struct Fingerprint(pub Vec<u8>);

impl Fingerprint {
    pub fn compress(self) -> CompressedFingerprint {
        unimplemented!()
    }
}

pub struct CompressedFingerprint(pub Vec<u8>);

impl CompressedFingerprint {
    pub fn encode(self) -> String {
        unimplemented!()
    }
}
