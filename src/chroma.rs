const MIN_FREQ: u32 = 28;
const MAX_FREQ: u32 = 3520;
const DEFAULT_SAMPLE_RATE: u32 = 11025;

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

        for idx in self.note_range.min_idx..self.note_range.max_idx {
            let note = self.note_range.get_note_for_idx(idx as usize);
            notes[note as usize] += frame[idx as usize];
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

    fn get_note_for_idx(&self, idx: usize) -> u8 {
        self.notes[idx]
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
    return (freq as f32 * size_per_frequency) as u32;
}

/// Converts an index in an FFT array to a frequency.
///
/// # Arguments
/// * `idx` - The index in the FFT array.
/// * `frame_size` - Size of the FFT frame.
/// * `sample_rate` - The maximum frequency.
fn idx_to_freq(idx: u32, frame_size: u32, sample_rate: u32) -> u32 {
    let frequency_per_size = (sample_rate as f32) / (frame_size as f32);
    return (idx as f32 * frequency_per_size) as u32;
}

/// Converts a frequency in Hz into a note.
///
/// # Returns
/// A value between 0 and 11. 0 corresponds to A and 11 to GSharp.
fn note_from_freq(frequency: u32) -> u8 {
    let octave = ((frequency as f64) / (440f64 / 16f64)).log2();
    (12f64 * (octave - octave.floor())) as u8
}

#[cfg(test)]
mod tests {
    use super::note_from_freq;
    use super::Chroma;

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
    fn test_note_from_freq() {
        assert_eq!(note_from_freq(28), 0);
        assert_eq!(note_from_freq(440), 0);
    }

    /*
    Test Generated With:

        #include <iostream>
        #include <math.h>
        using namespace std;

        inline double FreqToOctave(double freq, double base = 440.0 / 16.0)
        {
	        return log(freq / base) / log(2.0);
        }

        int Note(int frequency)
        {
	        double octave = FreqToOctave(frequency);
	        return (int)(12 * (octave - floor(octave))) % 12;
        }

        int main() {
	        for (int i = 28; i <= 3520; i += 100) {
		        std::cout <<
		            "assert_eq!(note_from_freq(" << i << "), " <<
		            Note(i) << ");" <<
		            std::endl;
	        }
        }
    */
    #[test]
    fn test_notes_from_freq() {
        assert_eq!(note_from_freq(28), 0);
        assert_eq!(note_from_freq(128), 2);
        assert_eq!(note_from_freq(228), 0);
        assert_eq!(note_from_freq(328), 6);
        assert_eq!(note_from_freq(428), 11);
        assert_eq!(note_from_freq(528), 3);
        assert_eq!(note_from_freq(628), 6);
        assert_eq!(note_from_freq(728), 8);
        assert_eq!(note_from_freq(828), 10);
        assert_eq!(note_from_freq(928), 0);
        assert_eq!(note_from_freq(1028), 2);
        assert_eq!(note_from_freq(1128), 4);
        assert_eq!(note_from_freq(1228), 5);
        assert_eq!(note_from_freq(1328), 7);
        assert_eq!(note_from_freq(1428), 8);
        assert_eq!(note_from_freq(1528), 9);
        assert_eq!(note_from_freq(1628), 10);
        assert_eq!(note_from_freq(1728), 11);
        assert_eq!(note_from_freq(1828), 0);
        assert_eq!(note_from_freq(1928), 1);
        assert_eq!(note_from_freq(2028), 2);
        assert_eq!(note_from_freq(2128), 3);
        assert_eq!(note_from_freq(2228), 4);
        assert_eq!(note_from_freq(2328), 4);
        assert_eq!(note_from_freq(2428), 5);
        assert_eq!(note_from_freq(2528), 6);
        assert_eq!(note_from_freq(2628), 6);
        assert_eq!(note_from_freq(2728), 7);
        assert_eq!(note_from_freq(2828), 8);
        assert_eq!(note_from_freq(2928), 8);
        assert_eq!(note_from_freq(3028), 9);
        assert_eq!(note_from_freq(3128), 9);
        assert_eq!(note_from_freq(3228), 10);
        assert_eq!(note_from_freq(3328), 11);
        assert_eq!(note_from_freq(3428), 11);
    }
}
