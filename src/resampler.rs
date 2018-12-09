use std::f64::consts::PI;

const FILTER_SHIFT: i32 = 15;

pub struct Resampler {
    phase_shift: i32,
    phase_mask: i32,
    linear: bool,
    filter_length: i32,
    filter_bank: Vec<i16>,
    src_incr: i32,
    ideal_dst_incr: i32,
    dst_incr: i32,
    index: i32,
    compensation_distance: i32,
    frac: i32,
}

impl Resampler {
    pub fn new(
        out_rate: i32,
        in_rate: i32,
        filter_size: i32,
        phase_shift: i32,
        linear: bool,
        cutoff: f64,
    ) -> Resampler {
        let factor = ((out_rate as f64) * cutoff / (in_rate as f64)).min(1.0);
        let phase_count = 1 << phase_shift;
        let filter_length = ((filter_size as f64 / factor).ceil() as i32).max(1);

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
    /// A tuple of the number of bytes consumed from `src` and the index of the
    /// last valid byte in `dst`.
    pub fn resample(&mut self, src: &[i16], dst: &mut [i16]) -> (usize, usize) {
        let mut consumed = 0;
        let mut last_dst_idx: i32 = 0;

        let mut index = self.index;
        let mut frac = self.frac;
        let mut dst_incr_frac = self.dst_incr % self.src_incr;
        let mut dst_incr = self.dst_incr / self.src_incr;

        let mut compensation_distance = self.compensation_distance;
        if compensation_distance == 0 && self.filter_length == 1 && self.phase_shift == 0 {
            let mut index2 = (index as i64) << 32;
            let incr = (1 << 32) * self.dst_incr as i64 / self.src_incr as i64;

            let dst_size = (dst.len() as i64).min(
                (src.len() as i32 - 1 - index) as i64 * (self.src_incr as i64)
                    / (self.dst_incr as i64),
            );

            for dst_idx in 0..(dst_size as usize) {
                dst[dst_idx] = src[(index2 >> 32) as usize];
                index2 += incr;

                last_dst_idx = last_dst_idx
            }

            frac += last_dst_idx * dst_incr_frac;
            index += last_dst_idx * dst_incr;
            index += frac / self.src_incr;
            frac %= self.src_incr;
        } else {
            for dst_index in 0..dst.len() {
                let filter = &self.filter_bank;
                let filter_offset = (self.filter_length * (index & self.phase_mask)) as usize;

                let sample_index = index >> self.phase_shift;
                let mut val: i32 = 0;

                if sample_index < 0 {
                    for i in 0..(self.filter_length as usize) {
                        val += (src[(sample_index + 1).abs() as usize % src.len()]
                            * filter[filter_offset + i]) as i32;
                    }
                } else if sample_index + self.filter_length > src.len() as i32 {
                    break;
                } else if self.linear {
                    let mut v2: i32 = 0;

                    for i in 0..self.filter_length {
                        val += (src[(sample_index + i) as usize] as i32)
                            * (filter[(filter_offset as i32 + i) as usize] as i32);
                        v2 += (src[sample_index as usize] as i32)
                            * (filter[(filter_offset as i32 + i + self.filter_length) as usize]
                                as i32);
                    }

                    val +=
                        ((v2 as i64 - val as i64) * (frac as i64) / (self.src_incr as i64)) as i32;
                } else {
                    for i in 0..self.filter_length {
                        val += (src[(sample_index + i) as usize] as i32)
                            * (filter[(filter_offset as i32 + i) as usize] as i32);
                    }
                }

                val = (val + (1 << (FILTER_SHIFT - 1))) >> FILTER_SHIFT;
                dst[dst_index] = if (val as u32 + 32768 as u32) > 65535 {
                    (val >> 31) ^ 32767
                } else {
                    val
                } as i16;

                frac += dst_incr_frac;
                index += dst_incr;
                if frac >= self.src_incr {
                    frac -= self.src_incr;
                    index += 1;
                }

                if dst_index as i32 + 1 == compensation_distance {
                    compensation_distance = 0;
                    dst_incr_frac = self.ideal_dst_incr % self.src_incr;
                    dst_incr = self.ideal_dst_incr / self.src_incr;
                }

                last_dst_idx = dst_index as i32
            }
        }

        consumed = index.max(0) >> self.phase_shift;
        if index >= 0 {
            index &= self.phase_mask;
        }

        if compensation_distance != 0 {
            compensation_distance -= last_dst_idx
        }

        self.frac = frac;
        self.index = index;
        self.dst_incr = dst_incr_frac + self.src_incr * dst_incr;
        self.compensation_distance = compensation_distance;

        (consumed as usize, last_dst_idx as usize)
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
    tap_count: i32,
    phase_count: i32,
    scale: f64,
) {
    let tap_count = tap_count as usize;
    let phase_count = phase_count as usize;

    let mut x = 0.0;
    let mut y = 0.0;
    let mut w = 0.0;

    let mut tab = vec![0.0f64; tap_count];
    let center = (tap_count - 1) / 2;

    factor = factor.min(1.0);

    for phase in 0..phase_count {
        let mut norm = 0.0;

        for i in 0..tap_count {
            x = PI
                * ((i as i32 - center as i32) as f64 - phase as f64 / phase_count as f64)
                * factor;
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
