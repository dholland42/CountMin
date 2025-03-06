pub fn density <const R: usize, const C: usize> (sketch: &[[f64; C]; R], rmask: &[bool; R], cmask: &[bool; C]) -> f64 {
    let mut sum: f64 = 0.;
    for (r, _) in rmask.iter().enumerate().take(R).filter(|(_, val)| **val) {
        for (c, _) in cmask.iter().enumerate().take(C).filter(|(_, val)| **val) {
            sum += sketch[r][c];
        }
    }
    sum / f64::sqrt(rmask.iter().filter(|v| **v).count() as f64 * cmask.iter().filter(|v| **v).count() as f64)
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
        assert_eq!(density(&items, &rmask, &cmask), 3. / f64::sqrt(3.));
        
    }
}
