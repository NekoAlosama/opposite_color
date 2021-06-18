use oklab::*;

#[inline]
fn difference(lab1: Oklab, lab2: Oklab) -> f32 {
    // HyAB color difference formula
    // Absolute difference between L, Euclidian distance between a and b
    // Made for L*a*b*, but probably works here
    (lab1.l - lab2.l).abs() + (lab1.a - lab2.a).hypot(lab1.b - lab2.b)
}

fn opposite(rgb: RGB<u8>) -> (RGB<u8>, f32) {
    // Generates the opposite color of a given RGB<u8>
    // Returns the opposite color and the difference

    let input_lab = srgb_to_oklab(rgb);
    let mut max = 0.0;

    // L ~= 0.5, a ~= 0.0, b ~= 0.0
    // If your result is this, something went wrong
    let mut saved = RGB {
        r: 99,
        g: 99,
        b: 99,
    };

    // All opposite colors are observed to be 1-bit colors
    // So, only 0 and 255 are possible color channel values
    for r in &[0, 255] {
        for g in &[0, 255] {
            for b in &[0, 255] {
                let test = RGB {
                    r: *r,
                    g: *g,
                    b: *b,
                };
                let test_lab = srgb_to_oklab(test);

                let delta = difference(input_lab, test_lab);

                if delta > max {
                    max = delta;
                    saved = test;
                }
            }
        }
    }

    (saved, max)
}

fn main() {
    let input = RGB {
        r: 255,
        g: 255,
        b: 0,
    };

    let color = opposite(input);

    println!("Opposite: {:?}", color.0);
    println!("Delta: {:?}", color.1);
}
