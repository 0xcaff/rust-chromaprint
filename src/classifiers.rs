use filter::Filter;
use quantizer::Quantizer;

pub type Classifier = (Filter, Quantizer);
pub type Classifiers = [Classifier; 16];

pub fn get_default_classifier() -> Classifiers {
    [
        (
            Filter::new(0, 4, 3, 15),
            Quantizer::new(1.98215, 2.35817, 2.63523),
        ),
        (
            Filter::new(4, 4, 6, 15),
            Quantizer::new(-1.03809, -0.651211, -0.282167),
        ),
        (
            Filter::new(1, 0, 4, 16),
            Quantizer::new(-0.298702, 0.119262, 0.558497),
        ),
        (
            Filter::new(3, 8, 2, 12),
            Quantizer::new(-0.105439, 0.0153946, 0.135898),
        ),
        (
            Filter::new(3, 4, 4, 8),
            Quantizer::new(-0.142891, 0.0258736, 0.200632),
        ),
        (
            Filter::new(4, 0, 3, 5),
            Quantizer::new(-0.826319, -0.590612, -0.368214),
        ),
        (
            Filter::new(1, 2, 2, 9),
            Quantizer::new(-0.557409, -0.233035, 0.0534525),
        ),
        (
            Filter::new(2, 7, 3, 4),
            Quantizer::new(-0.0646826, 0.00620476, 0.0784847),
        ),
        (
            Filter::new(2, 6, 2, 16),
            Quantizer::new(-0.192387, -0.029699, 0.215855),
        ),
        (
            Filter::new(2, 1, 3, 2),
            Quantizer::new(-0.0397818, -0.00568076, 0.0292026),
        ),
        (
            Filter::new(5, 10, 1, 15),
            Quantizer::new(-0.53823, -0.369934, -0.190235),
        ),
        (
            Filter::new(3, 6, 2, 10),
            Quantizer::new(-0.124877, 0.0296483, 0.139239),
        ),
        (
            Filter::new(2, 1, 1, 14),
            Quantizer::new(-0.101475, 0.0225617, 0.231971),
        ),
        (
            Filter::new(3, 5, 6, 4),
            Quantizer::new(-0.0799915, -0.00729616, 0.063262),
        ),
        (
            Filter::new(1, 9, 2, 12),
            Quantizer::new(-0.272556, 0.019424, 0.302559),
        ),
        (
            Filter::new(3, 4, 2, 14),
            Quantizer::new(-0.164292, -0.0321188, 0.0846339),
        ),
    ]
}
