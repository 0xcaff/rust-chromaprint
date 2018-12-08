use rolling_integral_image::RollingIntegralImage;

pub struct Filter {
    type_id: u8,
    y: usize,
    height: usize,
    width: usize,
}

impl Filter {
    pub fn new(type_id: u8, y: usize, height: usize, width: usize) -> Filter {
        Filter {
            type_id,
            y,
            height,
            width,
        }
    }

    pub fn apply(&self, image: &RollingIntegralImage, x: usize) -> f64 {
        let (a, b) = match self.type_id {
            0 => filter0(image, x, self.y, self.width, self.height),
            1 => filter1(image, x, self.y, self.width, self.height),
            2 => filter2(image, x, self.y, self.width, self.height),
            3 => filter3(image, x, self.y, self.width, self.height),
            4 => filter4(image, x, self.y, self.width, self.height),
            5 => filter5(image, x, self.y, self.width, self.height),
            _ => (0.0, 0.0),
        };

        subtract_log(a, b)
    }
}

fn subtract_log(a: f64, b: f64) -> f64 {
    ((1.0 + a) / (1.0 + b)).ln()
}

fn filter0(image: &RollingIntegralImage, x: usize, y: usize, w: usize, h: usize) -> (f64, f64) {
    // oooooooooooooooo
    // oooooooooooooooo
    // oooooooooooooooo
    // oooooooooooooooo

    (image.area(x, y, x + w, y + h), 0.0)
}

fn filter1(image: &RollingIntegralImage, x: usize, y: usize, w: usize, h: usize) -> (f64, f64) {
    // ................
    // ................
    // oooooooooooooooo
    // oooooooooooooooo

    let h_2 = h / 2;

    (
        image.area(x, y + h_2, x + w, y + h),
        image.area(x, y, x + w, y + h_2),
    )
}

fn filter2(image: &RollingIntegralImage, x: usize, y: usize, w: usize, h: usize) -> (f64, f64) {
    // .......ooooooooo
    // .......ooooooooo
    // .......ooooooooo
    // .......ooooooooo

    let w_2 = w / 2;

    (
        image.area(x + w_2, y, x + w, y + h),
        image.area(x, y, x + w_2, y + h),
    )
}

fn filter3(image: &RollingIntegralImage, x: usize, y: usize, w: usize, h: usize) -> (f64, f64) {
    // .......ooooooooo
    // .......ooooooooo
    // ooooooo.........
    // ooooooo.........

    let w_2 = w / 2;
    let h_2 = h / 2;

    (
        image.area(x, y + h_2, x + w_2, y + h) + image.area(x + w_2, y, x + w, y + h_2),
        image.area(x, y, x + w_2, y + h_2) + image.area(x + w_2, y + h_2, x + w, y + h),
    )
}

fn filter4(image: &RollingIntegralImage, x: usize, y: usize, w: usize, h: usize) -> (f64, f64) {
    // ................
    // oooooooooooooooo
    // ................

    let h_3 = h / 2;

    (
        image.area(x, y + h_3, x + w, y + 2 * h_3),
        image.area(x, y, x + w, y + h_3) + image.area(x, y + 2 * h_3, x + w, y + h),
    )
}

fn filter5(image: &RollingIntegralImage, x: usize, y: usize, w: usize, h: usize) -> (f64, f64) {
    // .....oooooo.....
    // .....oooooo.....
    // .....oooooo.....
    // .....oooooo.....

    let w_3 = w / 2;

    (
        image.area(x + w_3, y, x + 2 * w_3, y + h),
        image.area(x, y, x + w_3, y + h) + image.area(x + 2 * w_3, y, x + w, y + h),
    )
}

#[cfg(test)]
mod tests {
    use super::{filter0, filter1, filter2, filter3, filter4, filter5, subtract_log, Filter};
    use rolling_integral_image::RollingIntegralImage;

    trait PairDifference<T> {
        fn difference(&self) -> T;
    }

    impl PairDifference<f64> for (f64, f64) {
        fn difference(&self) -> f64 {
            self.0 - self.1
        }
    }

    #[test]
    fn test_filter() {
        let mut image = RollingIntegralImage::new(2);
        image.add_row([0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        image.add_row([2.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        let filter = Filter::new(0, 0, 1, 1);

        assert_eq!(0.0, filter.apply(&image, 0));
        assert_eq!(2.0, image.area(1, 0, 2, 1));
        assert_abs_diff_eq!(1.0986123, filter.apply(&image, 1), epsilon = 1e-7);
    }

    #[test]
    fn compare_subtract_log() {
        assert_abs_diff_eq!(0.4054651, subtract_log(2.0, 1.0), epsilon = 1e-7);
    }

    #[test]
    fn test_filter0() {
        let mut image = RollingIntegralImage::new(3);
        image.add_row([1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        image.add_row([4.0, 5.0, 6.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        image.add_row([7.0, 8.0, 9.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        assert_eq!(1.0, filter0(&image, 0, 0, 1, 1).difference());
        assert_eq!(12.0, filter0(&image, 0, 0, 2, 2).difference());
        assert_eq!(45.0, filter0(&image, 0, 0, 3, 3).difference());
        assert_eq!(28.0, filter0(&image, 1, 1, 2, 2).difference());
        assert_eq!(9.0, filter0(&image, 2, 2, 1, 1).difference());
        assert_eq!(12.0, filter0(&image, 0, 0, 3, 1).difference());
        assert_eq!(6.0, filter0(&image, 0, 0, 1, 3).difference());
    }

    #[test]
    fn test_filter1() {
        let mut image = RollingIntegralImage::new(3);
        image.add_row([1.0, 2.1, 3.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        image.add_row([3.1, 4.1, 5.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        image.add_row([6.0, 7.1, 8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        assert_eq!(1.0 - 0.0, filter1(&image, 0, 0, 1, 1).difference());
        assert_eq!(4.1 - 0.0, filter1(&image, 1, 1, 1, 1).difference());
        assert_eq!(2.1 - 1.0, filter1(&image, 0, 0, 1, 2).difference());
        assert_eq!(
            (2.1 + 4.1) - (1.0 + 3.1),
            filter1(&image, 0, 0, 2, 2).difference()
        );
        assert_eq!(
            (2.1 + 4.1 + 7.1) - (1.0 + 3.1 + 6.0),
            filter1(&image, 0, 0, 3, 2).difference()
        );
    }

    #[test]
    fn test_filter2() {
        let mut image = RollingIntegralImage::new(3);
        image.add_row([1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        image.add_row([3.0, 4.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        image.add_row([6.0, 7.0, 8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        assert_eq!(2.0, filter2(&image, 0, 0, 2, 1).difference());
        assert_eq!(4.0, filter2(&image, 0, 0, 2, 2).difference());
        assert_eq!(6.0, filter2(&image, 0, 0, 2, 3).difference());
    }

    #[test]
    fn test_filter3() {
        let mut image = RollingIntegralImage::new(3);
        image.add_row([1.0, 2.1, 3.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        image.add_row([3.1, 4.1, 5.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        image.add_row([6.0, 7.1, 8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        assert_abs_diff_eq!(
            0.1,
            filter3(&image, 0, 0, 2, 2).difference(),
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(
            0.1,
            filter3(&image, 1, 1, 2, 2).difference(),
            epsilon = 1e-7
        );
        assert_abs_diff_eq!(
            0.3,
            filter3(&image, 0, 1, 2, 2).difference(),
            epsilon = 1e-7
        );
    }

    #[test]
    fn test_filter4() {
        let mut image = RollingIntegralImage::new(3);
        image.add_row([1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        image.add_row([3.0, 4.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        image.add_row([6.0, 7.0, 8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        assert_eq!(-13.0, filter4(&image, 0, 0, 3, 3).difference());
    }

    #[test]
    fn test_filter5() {
        let mut image = RollingIntegralImage::new(3);
        image.add_row([1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        image.add_row([3.0, 4.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        image.add_row([6.0, 7.0, 8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        assert_eq!(-15.0, filter5(&image, 0, 0, 3, 3).difference());
    }
}
