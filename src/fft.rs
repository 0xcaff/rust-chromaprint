use rustfft::algorithm::Radix4;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFT;

use slicer::Slicer;
use std::sync::Arc;

const FRAME_SIZE: usize = 4096;

pub struct Fft {
    slicer: Slicer<i16>,
    fft: Arc<Radix4<f64>>,
}

impl Fft {
    pub fn new(overlap: usize) -> Fft {
        Fft {
            slicer: Slicer::new(FRAME_SIZE, FRAME_SIZE - overlap),
            fft: Arc::new(Radix4::new(FRAME_SIZE, false)),
        }
    }

    pub fn consume(&mut self, data: &[i16]) {
        let fft = self.fft.clone();
        self.slicer.process(data, move |vec| {
            let mut converted: Vec<Complex<f64>> = vec
                .into_iter()
                .map(|num| Complex::new(num as f64, 0.0))
                .collect();

            let mut output: Vec<Complex<f64>> = vec![Complex::zero(); FRAME_SIZE];
            fft.process(&mut converted, &mut output);

            let doubles: Vec<f64> = output.into_iter().map(|num| num.re).collect();
            let folded = fold_output(&doubles);

            // TODO: Forward to Next Step
        });
    }
}

pub fn fold_output(fft: &[f64]) -> Vec<f64> {
    let half_input = fft.len() / 2;
    let mut output = vec![0.0; half_input + 1];

    output[0] = fft[0] * fft[0];
    output[half_input] = fft[half_input] * fft[half_input];

    for idx in 1..half_input {
        let rev_idx = half_input - 1 - idx;

        output[idx] = fft[idx] * fft[idx] + fft[rev_idx] * fft[rev_idx]
    }

    output
}
