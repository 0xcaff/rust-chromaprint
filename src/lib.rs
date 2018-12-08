mod chroma;
mod chroma_filter;
mod chroma_normalize;
mod filter;
mod quantizer;
mod rolling_integral_image;

#[cfg(test)]
#[macro_use]
extern crate approx;

pub struct Fingerprinter;

impl Fingerprinter {
    pub fn new() -> Fingerprinter {
        Fingerprinter
    }

    pub fn feed(&mut self, raw_pcm: &[f32]) {
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
