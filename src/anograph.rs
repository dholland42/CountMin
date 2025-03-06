pub fn density <const R: usize, const C: usize> (sketch: &[[f64; C]; R], rmask: &[bool; R], cmask: &[bool; C]) -> f64 {
    let mut sum: f64 = 0.;
    for (r, _) in rmask.iter().enumerate().take(R).filter(|(_, val)| **val) {
        for (c, _) in cmask.iter().enumerate().take(C).filter(|(_, val)| **val) {
            sum += sketch[r][c];
        }
    }
    let denom = f64::sqrt(rmask.iter().filter(|v| **v).count() as f64 * cmask.iter().filter(|v| **v).count() as f64);
    
    // Make sure we handle the empty case.
    match denom {
        0. => 0.,
        _ => sum / denom
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_density() {
        let items = [
            [0., 1., 2.],
            [3., 4., 5.],
        ];
        let rmask = [true, false];
        let cmask = [true, true, true];
        // Given the mask, we should get (0 + 1 + 2) / sqrt(3).
        assert_eq!(density(&items, &rmask, &cmask), 3. / f64::sqrt(3.));
        // If things are empty, we get 0.
        assert_eq!(density(&[], &[], &[]), 0.);
        // If everything is masked, we get 0.
        assert_eq!(density(&items, &[false, false], &[false, false, false]), 0.);
        // If all rows are masked, we get 0.
        assert_eq!(density(&items, &[false, false], &[true, true, true]), 0.);
        // If all columns are masked, we get 0.
        assert_eq!(density(&items, &[true, true], &[false, false, false]), 0.);
    }
}
