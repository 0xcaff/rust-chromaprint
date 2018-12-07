const MAX_ROWS: usize = 257;

/// Computes rolling areas.
pub struct RollingIntegralImage {
    rows: Vec<[f64; 12]>,
    rows_count: usize,
    empty: bool,
}

impl RollingIntegralImage {
    pub fn new(max_rows: usize) -> RollingIntegralImage {
        RollingIntegralImage {
            rows: vec![[0.0; 12]; max_rows + 1],
            rows_count: 0,
            empty: true,
        }
    }

    pub fn rows(&self) -> usize {
        self.rows_count
    }

    pub fn area(&self, row1idx: usize, col1idx: usize, row2idx: usize, col2idx: usize) -> f64 {
        if row1idx == row2idx || col1idx == col2idx {
            return 0.0;
        }

        if row1idx == 0 {
            let row = &self.rows[(row2idx - 1) % self.rows.len()];

            if col1idx == 0 {
                return row[col2idx - 1];
            } else {
                return row[col2idx - 1] - row[col1idx - 1];
            }
        } else {
            let row1 = &self.rows[(row1idx - 1) % self.rows.len()];
            let row2 = &self.rows[(row2idx - 1) % self.rows.len()];

            if col1idx == 0 {
                return row2[col2idx - 1] - row1[col2idx - 1];
            } else {
                return row2[col2idx - 1] - row1[col2idx - 1] - row2[col1idx - 1]
                    + row1[col1idx - 1];
            }
        }
    }

    pub fn add_row(&mut self, row: [f64; 12]) {
        let next_row_idx = self.rows_count % self.rows.len();

        let mut sum = 0.0;
        for idx in 0..12 {
            sum += row[idx];
            self.rows[next_row_idx][idx] = sum;
        }

        if !self.empty {
            let last_row_idx = (self.rows_count - 1) % self.rows.len();

            for idx in 0..12 {
                self.rows[next_row_idx][idx] += self.rows[last_row_idx][idx]
            }
        }

        self.rows_count = self.rows_count + 1;
        self.empty = false
    }
}

#[cfg(test)]
mod tests {
    use super::RollingIntegralImage;

    #[test]
    fn test_all() {
        let mut image = RollingIntegralImage::new(4);
        image.add_row([1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        assert_eq!(1, image.rows());

        assert_eq!(1.0, image.area(0, 0, 1, 1));
        assert_eq!(2.0, image.area(0, 1, 1, 2));
        assert_eq!(3.0, image.area(0, 2, 1, 3));
        assert_eq!(1.0 + 2.0 + 3.0, image.area(0, 0, 1, 3));

        image.add_row([4.0, 5.0, 6.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        assert_eq!(2, image.rows());

        assert_eq!(4.0, image.area(1, 0, 2, 1));
        assert_eq!(5.0, image.area(1, 1, 2, 2));
        assert_eq!(6.0, image.area(1, 2, 2, 3));
        assert_eq!(1.0 + 2.0 + 3.0 + 4.0 + 5.0 + 6.0, image.area(0, 0, 2, 3));

        image.add_row([7.0, 8.0, 9.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        assert_eq!(3, image.rows());

        image.add_row([
            10.0, 11.0, 12.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ]);
        assert_eq!(4, image.rows());

        image.add_row([
            13.0, 14.0, 15.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ]);
        assert_eq!(5, image.rows());

        assert_eq!(4.0, image.area(1, 0, 2, 1));
        assert_eq!(5.0, image.area(1, 1, 2, 2));
        assert_eq!(6.0, image.area(1, 2, 2, 3));
        assert_eq!(13.0, image.area(4, 0, 5, 1));
        assert_eq!(14.0, image.area(4, 1, 5, 2));
        assert_eq!(15.0, image.area(4, 2, 5, 3));
        assert_eq!(
            ((4 + 5 + 6) + (7 + 8 + 9) + (10 + 11 + 12) + (13 + 14 + 15)) as f64,
            image.area(1, 0, 5, 3)
        );

        image.add_row([
            16.0, 17.0, 18.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ]);
        assert_eq!(6, image.rows());

        assert_eq!(7.0, image.area(2, 0, 3, 1));
        assert_eq!(8.0, image.area(2, 1, 3, 2));
        assert_eq!(9.0, image.area(2, 2, 3, 3));
        assert_eq!(16.0, image.area(5, 0, 6, 1));
        assert_eq!(17.0, image.area(5, 1, 6, 2));
        assert_eq!(18.0, image.area(5, 2, 6, 3));
        assert_eq!(
            ((7 + 8 + 9) + (10 + 11 + 12) + (13 + 14 + 15) + (16 + 17 + 18)) as f64,
            image.area(2, 0, 6, 3)
        );
    }
}
