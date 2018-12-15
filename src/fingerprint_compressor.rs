use bit_writer::BitWriter;

pub fn compress(fingerprint: &[u32], algorithm: u8) -> Vec<u8> {
    let mut normal_bits = Vec::new();
    let mut exceptional_bits = Vec::new();

    if !fingerprint.is_empty() {
        normal_bits.reserve(fingerprint.len());
        exceptional_bits.reserve(fingerprint.len() / 10);

        process_subfingerprint(fingerprint[0], &mut normal_bits, &mut exceptional_bits);

        for idx in 1..fingerprint.len() {
            process_subfingerprint(
                fingerprint[idx] ^ fingerprint[idx - 1],
                &mut normal_bits,
                &mut exceptional_bits,
            );
        }
    }

    let header_size = 4;
    let normal_bits_size = (normal_bits.len() * 3 + 7) / 8;
    let exceptional_bits_size = (exceptional_bits.len() * 5 + 7) / 8;
    let output_size = header_size + normal_bits_size + exceptional_bits_size;

    let mut output = vec![0u8; output_size];
    write_header(&mut output[..4], fingerprint.len(), algorithm);
    let normal_bits_output_size = BitWriter::write_all_into(&normal_bits, 3, &mut output[4..]);
    let exceptional_bits_output_size = BitWriter::write_all_into(
        &exceptional_bits,
        5,
        &mut output[(4 + normal_bits_output_size)..],
    );

    output.truncate(header_size + exceptional_bits_output_size + normal_bits_output_size);

    output
}

const K_NORMAL_BITS: u8 = 3;
const MAX_NORMAL_VALUE: u8 = (1 << K_NORMAL_BITS) - 1;

fn process_subfingerprint(mut x: u32, normal_bits: &mut Vec<u8>, exceptional_bits: &mut Vec<u8>) {
    let mut bit = 1;
    let mut last_bit = 0;

    while x != 0 {
        if x & 1 != 0 {
            let value = bit - last_bit;
            if value >= MAX_NORMAL_VALUE {
                normal_bits.push(MAX_NORMAL_VALUE);
                exceptional_bits.push(value - MAX_NORMAL_VALUE);
            } else {
                normal_bits.push(value);
            }

            last_bit = bit;
        }

        x >>= 1;
        bit += 1;
    }

    normal_bits.push(0);
}

fn write_header(output: &mut [u8], size: usize, algorithm: u8) {
    output[0] = algorithm;
    output[1] = ((size >> 16) & 255 as usize) as u8;
    output[2] = ((size >> 8) & 255 as usize) as u8;
    output[3] = (size & 255 as usize) as u8;
}

#[cfg(test)]
mod tests {
    use super::compress;

    #[test]
    fn one_item_one_bit() {
        assert_eq!(compress(&[1], 0), [0, 0, 0, 1, 1]);
    }

    #[test]
    fn one_item_three_bits() {
        assert_eq!(compress(&[7], 0), [0, 0, 0, 1, 73, 0]);
    }

    #[test]
    fn one_item_one_bit_except() {
        assert_eq!(compress(&[1 << 6], 0), [0, 0, 0, 1, 7, 0]);
    }

    #[test]
    fn one_item_one_bit_except_2() {
        assert_eq!(compress(&[1 << 8], 0), [0, 0, 0, 1, 7, 2]);
    }

    #[test]
    fn two_items() {
        assert_eq!(compress(&[1, 0], 0), [0, 0, 0, 2, 65, 0]);
    }

    #[test]
    fn two_items_no_change() {
        assert_eq!(compress(&[1, 1], 0), [0, 0, 0, 2, 1, 0]);
    }
}
