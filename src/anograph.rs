use std::hash::Hash;
use crate::hcms::HCountMin;


#[derive(Default)]
pub struct AnoGraph <const H: usize, const W: usize> {
    hcms: HCountMin<H, W>,
}

impl <const H: usize, const W: usize> AnoGraph <H, W> {
    pub fn score_graph(graph: &[[f64; W]; W]) -> f64 {
        let mut rmsk = [true; W];
        let mut cmsk = [true; W];
        let mut d = density(graph, &rmsk, &cmsk);
        for _ in 0..2*W {
            let (ridx, rv) = minrow(graph, &rmsk, &cmsk);
            let (cidx, cv) = mincol(graph, &rmsk, &cmsk);
            if rv < cv {
                rmsk[ridx] = false;
            } else {
                cmsk[cidx] = false;
            }
            let new_density = density(graph, &rmsk, &cmsk);
            if new_density > d {
                d = new_density;
            }
        }
        d
    }


    pub fn ingest<T: Hash>(&mut self, edges: &[(T, T)]) -> f64 {
        for (e1, e2) in edges.iter() {
            self.hcms.add(e1, e2);
        }
        let mut score = f64::MIN;
        for graph in self.hcms.sketch.iter() {
            let s = Self::score_graph(graph);
            if s > score {
                score = s;
            }
        }
        score
    }
}


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


pub fn maxrow <const R: usize, const C: usize> (sketch: &[[f64; C]; R], rmask: &[bool; R], cmask: &[bool; C]) -> (usize, f64) {
    let mut maxval: f64 = -1.;
    let mut maxidx: usize = 0;
    let mut rval: f64 = 0.;

    for (r, _) in rmask.iter().enumerate().take(R).filter(|(_, val)| **val) {
        rval = 0.;
        for (c, _) in cmask.iter().enumerate().take(C).filter(|(_, val)| **val) {
            rval += sketch[r][c];
        }
        if rval > maxval {
            maxval = rval;
            maxidx = r;
        }
    }
    (maxidx, rval)
}


pub fn maxcol <const R: usize, const C: usize> (sketch: &[[f64; C]; R], rmask: &[bool; R], cmask: &[bool; C]) -> (usize, f64) {
    let mut maxval: f64 = -1.;
    let mut maxidx: usize = 0;
    let mut cval: f64 = 0.;

    for (c, _) in cmask.iter().enumerate().take(C).filter(|(_, val)| **val) {
        cval = 0.;
        for (r, _) in rmask.iter().enumerate().take(R).filter(|(_, val)| **val) {
            cval += sketch[r][c];
        }
        if cval > maxval {
            maxval = cval;
            maxidx = c;
        }
    }
    (maxidx, cval)
}


pub fn minrow <const R: usize, const C: usize> (sketch: &[[f64; C]; R], rmask: &[bool; R], cmask: &[bool; C]) -> (usize, f64) {
    let mut minval: f64 = -1.;
    let mut minidx: usize = 0;
    let mut rval: f64 = 0.;

    for (r, _) in rmask.iter().enumerate().take(R).filter(|(_, val)| **val) {
        rval = 0.;
        for (c, _) in cmask.iter().enumerate().take(C).filter(|(_, val)| **val) {
            rval += sketch[r][c];
        }
        if rval < minval {
            minval = rval;
            minidx = r;
        }
    }
    (minidx, rval)
}


pub fn mincol <const R: usize, const C: usize> (sketch: &[[f64; C]; R], rmask: &[bool; R], cmask: &[bool; C]) -> (usize, f64) {
    let mut minval: f64 = -1.;
    let mut minidx: usize = 0;
    let mut cval: f64 = 0.;

    for (c, _) in cmask.iter().enumerate().take(C).filter(|(_, val)| **val) {
        cval = 0.;
        for (r, _) in rmask.iter().enumerate().take(R).filter(|(_, val)| **val) {
            cval += sketch[r][c];
        }
        if cval < minval {
            minval = cval;
            minidx = c;
        }
    }
    (minidx, cval)
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

    #[test]
    fn test_maxrow() {
        let items = [
            [0., 1., 2.],
            [3., 4., 5.],
            [6., 7., 8.],
        ];
        let rmask = [true, false, true];
        let cmask = [true, true, false];
        assert_eq!(maxrow(&items, &rmask, &cmask), (2, 13.));
    }

    #[test]
    fn test_maxcol() {
        let items = [
            [0., 1., 2.],
            [3., 4., 5.],
            [6., 7., 8.],
        ];
        let rmask = [true, false, true];
        let cmask = [true, true, false];
        assert_eq!(maxcol(&items, &rmask, &cmask), (1, 8.));
    }

    #[test]
    fn test_anograph_scoring() {
        let edges = [
            ("s1", "d1"),
            ("s1", "d2"),
            ("s1", "d3"),
        ];
        let mut ag = AnoGraph::<2, 32>::default();
        let score = ag.ingest(&edges);
        assert!(score > 0.);
    }

}
