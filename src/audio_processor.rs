use resampler::Resampler;
use slicer::Slicer;

const MAX_BUFFER_SIZE: usize = 1024 * 32;
const RESAMPLE_FILTER_LENGTH: i32 = 16;
const RESAMPLE_PHASE_SHIFT: i32 = 8;
const RESAMPLE_LINEAR: bool = false;
const RESAMPLE_SAMPLE_CUTOFF: f64 = 0.8;

pub struct AudioProcessor {
    slicer: Option<Slicer<i16>>,
    resampler: Resampler,
}

impl AudioProcessor {
    pub fn new(target_sample_rate: u16, input_sample_rate: u16) -> AudioProcessor {
        AudioProcessor {
            slicer: Some(Slicer::new(MAX_BUFFER_SIZE)),
            resampler: Resampler::new(
                target_sample_rate as i32,
                input_sample_rate as i32,
                RESAMPLE_FILTER_LENGTH,
                RESAMPLE_PHASE_SHIFT,
                RESAMPLE_LINEAR,
                RESAMPLE_SAMPLE_CUTOFF,
            ),
        }
    }

    pub fn feed<C: FnMut(Vec<i16>)>(&mut self, data: &[i16], mut consumer: C) {
        let mut slicer = self.slicer.take().unwrap();

        slicer.process(data, |src| {
            let (consumed_size, dst) = self.resample_slice(src);
            consumer(dst);
            consumed_size
        });

        self.slicer = Some(slicer);
    }

    /// Transcodes any un-transcoded samples and returns if any are left.
    pub fn flush(&mut self) -> Option<Vec<i16>> {
        let mut slicer = self.slicer.take().unwrap();
        let remaining = slicer.flush();
        self.slicer = Some(slicer);
        if remaining.is_empty() {
            return None;
        }

        let (_, dst) = self.resample_slice(remaining);
        Some(dst)
    }

    fn resample_slice(&mut self, src: Vec<i16>) -> (usize, Vec<i16>) {
        let mut dst = vec![0i16; MAX_BUFFER_SIZE];

        let (consumed_size, last_idx) = self.resampler.resample(&src, &mut dst);
        dst.truncate(last_idx + 1);

        (consumed_size, dst)
    }
}
