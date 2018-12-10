pub struct Chroma {
    note_range: NoteRange,
}

impl Chroma {
    pub fn new(min_freq: u32, max_freq: u32, frame_size: u32, sample_rate: u32) -> Chroma {
        Chroma {
            note_range: NoteRange::new(min_freq, max_freq, frame_size, sample_rate),
        }
    }

    pub fn handle_frame(&self, frame: &[f64]) -> [f64; 12] {
        let mut notes = [0f64; 12];
        let note_for_idx = self.note_range.notes();

        for idx in self.note_range.min_idx..self.note_range.max_idx {
            notes[note_for_idx[idx as usize] as usize] += frame[idx as usize];
        }

        notes
    }
}

struct NoteRange {
    min_idx: u32,
    max_idx: u32,

    notes: Vec<u8>,
}

impl NoteRange {
    fn new(min_freq: u32, max_freq: u32, frame_size: u32, sample_rate: u32) -> NoteRange {
        let min_idx = u32::max(1, freq_to_idx(min_freq, frame_size, sample_rate));
        let max_idx = u32::min(
            frame_size / 2,
            freq_to_idx(max_freq, frame_size, sample_rate),
        );
        let mut notes = vec![0u8; frame_size as usize];

        for idx in min_idx..max_idx {
            let freq = idx_to_freq(idx, frame_size, sample_rate);
            let note = note_from_freq(freq);

            notes[idx as usize] = note
        }

        NoteRange {
            min_idx,
            max_idx,
            notes,
        }
    }

    pub fn notes(&self) -> &[u8] {
        &self.notes
    }
}

/// Converts a frequency (Hz) to an index in an FFT array.
///
/// # Arguments
/// * `freq` - The frequency to convert to an index.
/// * `frame_size` - Size of an FFT frame. Returns a value in `[0, `frame_size`]`.
/// * `sample_rate` - The maximum frequency.
fn freq_to_idx(freq: u32, frame_size: u32, sample_rate: u32) -> u32 {
    let size_per_frequency = (frame_size as f32) / (sample_rate as f32);
    return (freq as f32 * size_per_frequency).round() as u32;
}

/// Converts an index in an FFT array to a frequency.
///
/// # Arguments
/// * `idx` - The index in the FFT array.
/// * `frame_size` - Size of the FFT frame.
/// * `sample_rate` - The maximum frequency.
fn idx_to_freq(idx: u32, frame_size: u32, sample_rate: u32) -> f64 {
    let frequency_per_size = (sample_rate as f64) / (frame_size as f64);
    return idx as f64 * frequency_per_size;
}

/// Converts a frequency in Hz into a note.
///
/// # Returns
/// A value between 0 and 11. 0 corresponds to A and 11 to GSharp.
fn note_from_freq(frequency: f64) -> u8 {
    let octave = ((frequency as f64) / (440f64 / 16f64)).log2();
    (12f64 * (octave - octave.floor())) as u8
}

#[cfg(test)]
mod tests {
    use super::freq_to_idx;
    use super::note_from_freq;
    use super::{Chroma, NoteRange};
    use chroma_normalize::normalize_vector;
    use test_data;

    #[test]
    fn test_freq_to_idx() {
        assert_eq!(freq_to_idx(3520, 4096, 11025), 1308);
    }

    #[test]
    fn chroma_normal_a() {
        let chroma = Chroma::new(10, 510, 256, 1000);
        let mut frame = [0.0f64; 128];
        frame[113] = 1.0;

        assert_eq!(
            chroma.handle_frame(&frame),
            [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        );
    }

    #[test]
    fn chroma_normal_g_sharp() {
        let chroma = Chroma::new(10, 510, 256, 1000);
        let mut frame = [0.0f64; 128];
        frame[112] = 1.0;

        assert_eq!(
            chroma.handle_frame(&frame),
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,]
        );
    }

    #[test]
    fn chroma_normal_b() {
        let chroma = Chroma::new(10, 510, 256, 1000);
        let mut frame = [0.0f64; 128];
        frame[64] = 1.0;

        assert_eq!(
            chroma.handle_frame(&frame),
            [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,]
        );
    }

    #[test]
    fn test_notes_freq() {
        const MIN_FREQ: u32 = 28;
        const MAX_FREQ: u32 = 3520;
        const DEFAULT_SAMPLE_RATE: u32 = 11025;
        const FRAME_SIZE: u32 = 4096;

        let note_range = NoteRange::new(MIN_FREQ, MAX_FREQ, FRAME_SIZE, DEFAULT_SAMPLE_RATE);
        assert_eq!(&test_data::get_notes()[..], note_range.notes());
    }

    #[test]
    fn test_chroma() {
        let fft_frames = test_data::get_fft_frames();
        const MIN_FREQ: u32 = 28;
        const MAX_FREQ: u32 = 3520;
        const FRAME_SIZE: u32 = 4096;
        const TARGET_SAMPLE_RATE: u32 = 11025;

        let expected = test_data::get_chroma_features();
        let chroma = Chroma::new(MIN_FREQ, MAX_FREQ, FRAME_SIZE, TARGET_SAMPLE_RATE);

        let features: Vec<_> = fft_frames
            .into_iter()
            .map(|frame| chroma.handle_frame(&frame))
            .collect();

        assert_eq!(expected, features);
    }
}
