pub fn normalize_vector(mut vector: [f64; 12]) -> [f64; 12] {
    let norm = euclidean_norm(&vector);
    if norm < 0.01 {
        [0.0; 12]
    } else {
        for idx in 0..12 {
            vector[idx] /= norm
        }

        vector
    }
}

pub fn euclidean_norm(vector: &[f64; 12]) -> f64 {
    let mut squares = 0.0;
    for value in vector {
        squares += value * value
    }

    if squares > 0.0 {
        squares.sqrt()
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::{euclidean_norm, normalize_vector};

    #[test]
    fn test_euclidean_norm() {
        let data = [0.1, 0.2, 0.4, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        assert_eq!(euclidean_norm(&data), 1.1);
    }

    #[test]
    fn test_normalize_vector() {
        let data = [0.1, 0.2, 0.4, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let expected = [
            0.090909, 0.181818, 0.363636, 0.909091, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ];

        let got = normalize_vector(data);

        for idx in 0..12 {
            assert_ulps_eq!(got[idx], expected[idx], epsilon = 1e-5);
        }
    }

    #[test]
    fn test_normalize_vector_near_zero() {
        let data = [
            0.0, 0.001, 0.002, 0.003, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ];
        let expected = [0.0f64; 12];

        assert_eq!(normalize_vector(data), expected);
    }

    #[test]
    fn test_normalize_vector_zero() {
        let data = [0.0f64; 12];
        let expected = [0.0f64; 12];

        assert_eq!(normalize_vector(data), expected);
    }
}
