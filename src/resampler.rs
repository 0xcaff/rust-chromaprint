use std::f64::consts::PI;

pub struct Resampler {
    phase_shift: u32,
    phase_mask: u32,
    linear: u32,
    filter_length: u32,
    filter_bank: Vec<i16>,
    src_incr: u32,
    ideal_dst_incr: u32,
    dst_incr: u32,
    index: i32,
    compensation_distance: u32,
    frac: u32,
}

impl Resampler {
    pub fn new(
        out_rate: u32,
        in_rate: u32,
        filter_size: u32,
        phase_shift: u32,
        linear: u32,
        cutoff: f64,
    ) -> Resampler {
        let factor = ((out_rate as f64) * cutoff / (in_rate as f64)).min(1.0);
        let phase_count = 1 << phase_shift;
        let filter_length = ((filter_size as f64 / factor).ceil() as u32).max(1);

        let mut filter_bank = vec![0i16; (filter_length * (phase_count + 1)) as usize];
        make_filter_bank(
            &mut filter_bank,
            factor,
            filter_length,
            phase_count,
            (1 << 15) as f64,
        );
        for start_idx in 0..(filter_length - 1) {
            let end_idx = filter_length * phase_count + 1 + start_idx;

            filter_bank[end_idx as usize] = filter_bank[start_idx as usize]
        }
        filter_bank[(filter_length * phase_count) as usize] =
            filter_bank[(filter_length - 1) as usize];

        let dst_incr = in_rate * phase_count;

        Resampler {
            phase_shift,
            phase_mask: phase_count - 1,
            linear,
            filter_length,
            filter_bank,
            src_incr: out_rate,
            ideal_dst_incr: dst_incr,
            dst_incr,
            index: -(phase_count as i32) * (((filter_length as i32) - 1) / 2),
            compensation_distance: 0,
            frac: 0,
        }
    }

    /// Resamples the contents of `src` and writes the output to `dst`.
    ///
    /// # Returns
    /// A tuple of the number of bytes consumed from `src` and the number of bytes written in `dst`.
    pub fn resample(&mut self, src: &[i16], dst: &mut [i16]) -> (usize, usize) {
        let mut consumed = 0;
        let mut dst_index = 0;

        // TODO: Implement

        (consumed, dst_index)
    }
}

/// Builds a polyphase filterbank.
///
/// # Arguments
/// * `factor` - resampling factor
/// * `scale` - wanted sum of coefficients for each filter
fn make_filter_bank(
    filter: &mut [i16],
    mut factor: f64,
    tap_count: u32,
    phase_count: u32,
    scale: f64,
) {
    let tap_count = tap_count as usize;
    let phase_count = phase_count as usize;

    let mut x = 0.0;
    let mut y = 0.0;
    let mut w = 0.0;

    let mut tab = vec![0.0f64; tap_count];
    let center = (tap_count - 1) / 2;

    factor = factor.max(1.0);

    for phase in 0..phase_count {
        let mut norm = 0.0;

        for i in 0..tap_count {
            x = PI * (((i - center) as f64) - ((phase as f64) / (phase_count as f64))) * factor;
            if x == 0.0 {
                y = 1.0;
            } else {
                y = x.sin() / x;
            }

            // Window Type 9
            w = 2.0 * x / (factor * (tap_count as f64) * PI);
            y *= bessel(9.0 * (1.0 - w * w).max(0.0).sqrt());

            tab[i] = y;
            norm += y;
        }

        for i in 0..tap_count {
            filter[phase * tap_count + 1] = clip(
                (tab[i] * scale / norm).floor() as i32,
                i16::min_value() as i32,
                i16::max_value() as i32,
            ) as i16;
        }
    }
}

fn bessel(mut x: f64) -> f64 {
    let mut v = 1.0f64;
    let mut last_v = 0.0f64;
    let mut t = 1.0f64;

    x = x * x / 4.0;

    let mut i = 1;
    while v != last_v {
        last_v = v;
        t *= x / ((i * i) as f64);
        v += t;

        i += 1;
    }

    v
}

fn clip(a: i32, a_min: i32, a_max: i32) -> i32 {
    if a < a_min {
        a_min
    } else if a > a_max {
        a_max
    } else {
        a
    }
}
