use audio_processor::AudioProcessor;
use chroma::Chroma;
use chroma_normalize::normalize_vector;
use classifiers;
use fft::Fft;
use fingerprint_calculator::FingerprintCalculator;
use fingerprint_compressor;

pub const TARGET_SAMPLE_RATE: u16 = 11025;
pub const MIN_FREQ: u32 = 28;
pub const MAX_FREQ: u32 = 3520;
pub const FRAME_SIZE: usize = 4096;

pub struct Fingerprinter {
    audio_processor: Option<AudioProcessor>,
    fft: Option<Fft>,
    chroma: Chroma,
    fingerprint_calculator: FingerprintCalculator,
}

impl Fingerprinter {
    pub fn new(sample_rate: u16) -> Fingerprinter {
        Fingerprinter {
            audio_processor: Some(AudioProcessor::new(TARGET_SAMPLE_RATE, sample_rate)),
            fft: Some(Fft::new()),
            chroma: Chroma::new(
                MIN_FREQ,
                MAX_FREQ,
                FRAME_SIZE as u32,
                TARGET_SAMPLE_RATE as u32,
            ),
            fingerprint_calculator: FingerprintCalculator::new(
                classifiers::get_default_classifier(),
            ),
        }
    }

    pub fn feed(&mut self, raw_pcm: &[i16]) {
        let mut audio_processor = self.audio_processor.take().unwrap();
        let mut fft = self.fft.take().unwrap();

        audio_processor.feed(raw_pcm, |samples| {
            self.handle_resampled(samples, &mut fft);
        });

        self.fft = Some(fft);
        self.audio_processor = Some(audio_processor);
    }

    pub fn finish(&mut self) {
        let mut fft = self.fft.take().unwrap();

        self.audio_processor
            .as_mut()
            .unwrap()
            .flush()
            .map(|last_samples| self.handle_resampled(last_samples, &mut fft));

        self.fft = Some(fft);
    }

    fn handle_resampled(&mut self, samples: Vec<i16>, fft: &mut Fft) {
        fft.consume(&samples, |frame| {
            let features = self.chroma.handle_frame(&frame);
            let normalized_features = normalize_vector(features);

            self.fingerprint_calculator.consume(normalized_features);
        });
    }

    pub fn fingerprint(&self) -> Fingerprint {
        Fingerprint(self.fingerprint_calculator.fingerprint())
    }
}

pub struct Fingerprint<'a>(pub &'a [u32]);

impl<'a> Fingerprint<'a> {
    pub fn compress(&self) -> Vec<u8> {
        fingerprint_compressor::compress(self.0, 1)
    }
}
