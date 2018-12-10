use rustfft::algorithm::Radix4;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFT;

use slicer::Slicer;
use std::f64::consts::PI;

const FRAME_SIZE: usize = 4096;

pub struct Fft {
    slicer: Option<Slicer<i16>>,
    fft: Radix4<f64>,
    hamming_window: Vec<f64>,
}

impl Fft {
    pub fn new(overlap: usize) -> Fft {
        Fft {
            slicer: Some(Slicer::new(FRAME_SIZE, FRAME_SIZE - overlap)),
            fft: Radix4::new(FRAME_SIZE, false),
            hamming_window: prepare_hamming_window(FRAME_SIZE, 1.0 / ::std::i16::MAX as f64),
        }
    }

    pub fn consume<C: FnMut(Vec<f64>)>(&mut self, data: &[i16], mut consumer: C) {
        let mut slicer = self.slicer.take().unwrap();

        slicer.process(data, |vec| {
            let mut converted: Vec<Complex<f64>> = vec
                .into_iter()
                .enumerate()
                .map(|(idx, data)| self.hamming_window[idx] * (data as f64))
                .map(|num| Complex::new(num as f64, 0.0))
                .collect();

            let mut output: Vec<Complex<f64>> = vec![Complex::zero(); FRAME_SIZE];
            self.fft.process(&mut converted, &mut output);

            let folded = fold_output(&output);

            consumer(folded);
        });

        self.slicer = Some(slicer);
    }
}

pub fn fold_output(fft: &[Complex<f64>]) -> Vec<f64> {
    let half_input = fft.len() / 2;
    let mut output = vec![0.0; half_input + 1];

    for idx in 0..(half_input + 1) {
        output[idx] = fft[idx].re * fft[idx].re + fft[idx].im * fft[idx].im;
    }

    output
}

fn prepare_hamming_window(size: usize, scale: f64) -> Vec<f64> {
    let mut result = vec![0.0; size];

    for idx in 0..size {
        result[idx] = scale * (0.54 - 0.46 * (idx as f64 * 2.0 * PI / (size as f64 - 1.0)).cos())
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{prepare_hamming_window, Fft, FRAME_SIZE};
    use std::error::Error;
    use std::path::PathBuf;
    use test_data;
    use tests::load_audio_file;

    const OVERLAP: usize = FRAME_SIZE - FRAME_SIZE / 3;

    #[test]
    fn test_prepare_hamming_window() {
        let expected = vec![
            0.08,
            0.187619556165,
            0.460121838273,
            0.77,
            0.972258605562,
            0.972258605562,
            0.77,
            0.460121838273,
            0.187619556165,
            0.08,
        ];

        let window = prepare_hamming_window(10, 1.0);
        for idx in 0..10 {
            assert_abs_diff_eq!(expected[idx], window[idx], epsilon = 1e-9);
        }
    }

    #[test]
    fn test_fft() -> Result<(), Box<dyn Error>> {
        let samples = load_audio_file(
            &PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("./test_data/test_stero_44100_resampled_11025.raw"),
        )?;

        let mut fft = Fft::new(OVERLAP);
        let mut frames = Vec::new();
        fft.consume(&samples, |frame| {
            frames.push(frame);
        });

        let expected = test_data::get_fft_frames();
        for idx in 0..expected.len() {
            let expected_row = &expected[idx];
            let actual_row = &frames[idx];

            for row_idx in 0..expected_row.len() {
                // This epsilon is kinda large because of the differing FFT implementations used.
                // This is compared to the C thing which uses some implementation and the Rust thing
                // which uses rustfft.
                assert_ulps_eq!(expected_row[row_idx], actual_row[row_idx], epsilon = 1e-2);
            }
        }

        Ok(())
    }
}
