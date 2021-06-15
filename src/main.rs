use oklab::*;

fn difference(lab1: Oklab, lab2: Oklab) -> f32 {
    // HyAB color difference formula
    // Absolute difference between L, Euclidian distance between a and b
    // Made for L*a*b*, but probably works here
    (lab1.l - lab2.l).abs() + (lab1.a - lab2.a).hypot(lab1.b - lab2.b)
}

fn main() {
    // Value to compare with
    let input = RGB {
        r: 0,
        g: 0,
        b: 0
    };
    let input_lab = srgb_to_oklab(input);

    let mut max = 0.0;
    let mut saved = RGB { r: 99, g: 99, b: 99 }; // L ~= 0.5

    for r in 0..=255 {
        for g in 0..=255 {
            for b in 0..=255 {
                let test = RGB { r, g, b };
                let test_lab = srgb_to_oklab(test);

                let delta = difference(input_lab, test_lab);

                if delta > max {
                    max = delta;
                    saved = RGB { r, g, b };
                }
            }
        }
    }

    println!("Opposite: {:?}", saved);
    println!("Delta: {:?}", max);
}
