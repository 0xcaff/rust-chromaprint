use audio_processor::AudioProcessor;
use chroma::Chroma;
use chroma_normalize::normalize_vector;
use fft::Fft;
use resampler::Resampler;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::path::Path;
use std::path::PathBuf;

const FRAME_SIZE: usize = 4096;
const OVERLAP: usize = FRAME_SIZE - FRAME_SIZE / 3;

const MIN_FREQ: u32 = 28;
const MAX_FREQ: u32 = 3520;
const TARGET_SAMPLE_RATE: i32 = 11025;
const INPUT_SAMPLE_RATE: i32 = 44100;
const RESAMPLE_FILTER_LENGTH: i32 = 16;
const RESAMPLE_PHASE_SHIFT: i32 = 8;
const RESAMPLE_LINEAR: bool = false;
const RESAMPLE_SAMPLE_CUTOFF: f64 = 0.8;

#[test]
fn test_chromaprint() -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("./test_data/test_stereo_44100.raw");
    let samples = load_stero_audio_file(&path)?;

    let mut resampler = Resampler::new(
        TARGET_SAMPLE_RATE,
        INPUT_SAMPLE_RATE,
        RESAMPLE_FILTER_LENGTH,
        RESAMPLE_PHASE_SHIFT,
        RESAMPLE_LINEAR,
        RESAMPLE_SAMPLE_CUTOFF,
    );

    let mut fft = Fft::new(OVERLAP);
    let chroma = Chroma::new(
        MIN_FREQ,
        MAX_FREQ,
        FRAME_SIZE as u32,
        TARGET_SAMPLE_RATE as u32,
    );
    let mut image = Vec::new();

    let mut resampled = vec![0i16; samples.len()];
    let (_, last_idx) = resampler.resample(&samples, &mut resampled);

    fft.consume(&resampled[..(last_idx + 1)], |frame| {
        let chroma_features = chroma.handle_frame(&frame);
        let chroma_features_normalized = normalize_vector(chroma_features);
        image.push(chroma_features_normalized);
    });

    let expected = vec![
        [
            0.155444, 0.268618, 0.474445, 0.159887, 0.1761, 0.423511, 0.178933, 0.34433, 0.360958,
            0.30421, 0.200217, 0.17072,
        ],
        [
            0.159809, 0.238675, 0.286526, 0.166119, 0.225144, 0.449236, 0.162444, 0.371875,
            0.259626, 0.483961, 0.24491, 0.17034,
        ],
        [
            0.156518, 0.271503, 0.256073, 0.152689, 0.174664, 0.52585, 0.141517, 0.253695,
            0.293199, 0.332114, 0.442906, 0.170459,
        ],
        [
            0.154183, 0.38592, 0.497451, 0.203884, 0.362608, 0.355691, 0.125349, 0.146766,
            0.315143, 0.318133, 0.172547, 0.112769,
        ],
        [
            0.201289, 0.42033, 0.509467, 0.259247, 0.322772, 0.325837, 0.140072, 0.177756,
            0.320356, 0.228176, 0.148994, 0.132588,
        ],
        [
            0.187921, 0.302804, 0.46976, 0.302809, 0.183035, 0.228691, 0.206216, 0.35174, 0.308208,
            0.233234, 0.316017, 0.243563,
        ],
        [
            0.213539, 0.240346, 0.308664, 0.250704, 0.204879, 0.365022, 0.241966, 0.312579,
            0.361886, 0.277293, 0.338944, 0.290351,
        ],
        [
            0.227784, 0.252841, 0.295752, 0.265796, 0.227973, 0.451155, 0.219418, 0.272508,
            0.376082, 0.312717, 0.285395, 0.165745,
        ],
        [
            0.168662, 0.180795, 0.264397, 0.225101, 0.562332, 0.33243, 0.236684, 0.199847,
            0.409727, 0.247569, 0.21153, 0.147286,
        ],
        [
            0.0491864, 0.0503369, 0.130942, 0.0505802, 0.0694409, 0.0303877, 0.0389852, 0.674067,
            0.712933, 0.05762, 0.0245158, 0.0389336,
        ],
        [
            0.0814379, 0.0312366, 0.240546, 0.134609, 0.063374, 0.0466124, 0.0752175, 0.657041,
            0.680085, 0.0720311, 0.0249404, 0.0673359,
        ],
        [
            0.139331, 0.0173442, 0.49035, 0.287237, 0.0453947, 0.0873279, 0.15423, 0.447475,
            0.621502, 0.127166, 0.0355933, 0.141163,
        ],
        [
            0.115417, 0.0132515, 0.356601, 0.245902, 0.0283943, 0.0588233, 0.117077, 0.499376,
            0.715366, 0.100398, 0.0281382, 0.0943482,
        ],
        [
            0.047297, 0.0065354, 0.181074, 0.121455, 0.0135504, 0.030693, 0.0613105, 0.631705,
            0.73548, 0.0550565, 0.0128093, 0.0460393,
        ],
    ];

    for row_idx in 0..expected.len() {
        for col_idx in 0..12 {
            assert_abs_diff_eq!(
                expected[row_idx][col_idx],
                image[row_idx][col_idx],
                epsilon = 1e-6
            );
        }
    }

    Ok(())
}

pub fn load_stero_audio_file<T: AsRef<Path>>(path: T) -> Result<Vec<i16>, Box<dyn Error>> {
    Ok(load_audio_file(&path)?
        .chunks(2)
        .map(|chunk| (chunk[0] + chunk[1]) / (chunk.len() as i16))
        .collect())
}

pub fn load_audio_file<T: AsRef<Path>>(path: T) -> Result<Vec<i16>, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer
        .chunks(2)
        .map(|chunk| from_ne_bytes([chunk[0], chunk[1]]))
        .collect())
}

pub fn from_ne_bytes(bytes: [u8; 2]) -> i16 {
    unsafe { mem::transmute(bytes) }
}
