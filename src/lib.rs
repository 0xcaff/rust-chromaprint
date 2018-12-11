extern crate rustfft;

mod audio_processor;
mod bit_writer;
mod chroma;
mod chroma_filter;
mod chroma_normalize;
mod classifiers;
mod combined_buffer;
mod fft;
mod filter;
mod fingerprint_calculator;
mod fingerprint_compressor;
mod quantizer;
mod resampler;
mod rolling_integral_image;
mod slicer;

#[cfg(test)]
#[macro_use]
extern crate approx;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_data;

mod fingerprinter;
