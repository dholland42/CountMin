use std::hash::{DefaultHasher, Hash, Hasher};
use rand::prelude::*;
use rand::rngs::SmallRng;


#[derive(Debug)]
pub struct HCountMin <const H: usize, const W: usize> {
    pub sketch: [[[f64; W]; W]; H],
    offsets_a: [u32; H],
    offsets_b: [u32; H],
}


impl<const H: usize, const W: usize> Default for HCountMin <H, W> {
    /// Add a default implementation for CountMin
    ///
    /// # Examples
    ///
    /// ```
    /// use countmin::HCountMin;
    ///
    /// let mut mc = HCountMin::<2, 10>::default();
    /// mc.add(&"Hi", &"there");
    /// assert_eq!(mc.getcount(&"Hi", &"there"), Some(1.));
    /// ```
    fn default() -> Self {
        Self::new(None)
    }
}


impl <const H: usize, const W: usize> HCountMin <H, W> {

    // Create a new MinCount sketch
    pub fn new(seed: Option<u64>) -> HCountMin <H, W> {
        let mut rng = if let Some(s) = seed {
            SmallRng::seed_from_u64(s)
        } else {
            SmallRng::from_rng(&mut rand::rng())
        };
        let offsets_a: Vec<u32> = (0..H).map(|_| rng.random::<u32>() % (W as u32 - 1) + 1).collect();
        let offsets_b: Vec<u32> = (0..H).map(|_| rng.random::<u32>() % W as u32).collect();
        HCountMin {
            sketch: [[[0.; W]; W]; H],
            offsets_a: offsets_a.try_into().unwrap(),
            offsets_b: offsets_b.try_into().unwrap(),
        }
    }

    // A helper function to generate item hashes as u64
    pub fn hash<T: Hash>(&self, item: &T) -> u64 {
        let mut h = DefaultHasher::new();
        item.hash(&mut h);
        h.finish()
    }

    // Add an item to the sketch by hashing it into our buckets and incrementing
    // the count in each bucket.
    pub fn add<T: Hash>(&mut self, source: &T, target: &T) {
        let source_h = self.hash(source);
        let target_h = self.hash(target);
        self.add_idx(source_h, target_h);
    }

    pub fn add_idx(&mut self, s: u64, t: u64) {
        for i in 0..H {
            // Note that we need to use wrapping arithmetic here
            let sidx = (s).wrapping_mul(self.offsets_a[i] as u64).wrapping_add(self.offsets_b[i] as u64) % W as u64;
            let tidx = (t).wrapping_mul(self.offsets_a[i] as u64).wrapping_add(self.offsets_b[i] as u64) % W as u64;
            self.sketch[i][sidx as usize][tidx as usize] += 1.;
        }
    }

    // Get the estimate for the number of times an item as heen seen so far by
    // hashing it into our buckets and then taking the minimum of all those values.
    // We use the minimum value as our estimate as we will only ever _over_count
    // items due to hash collisions.
    pub fn getcount<T: Hash>(&mut self, source: &T, target: &T) -> Option<f64> {
        let source_h = self.hash(source);

        let target_h = self.hash(target);
        self.getcount_idx(source_h, target_h)
    }

    pub fn getcount_idx(&mut self, s: u64, t: u64) -> Option<f64> {
        let m = (0..H).map(|i| {
            let sidx = (s).wrapping_mul(self.offsets_a[i] as u64).wrapping_add(self.offsets_b[i] as u64) % W as u64;
            let tidx = (t).wrapping_mul(self.offsets_a[i] as u64).wrapping_add(self.offsets_b[i] as u64) % W as u64;
            self.sketch[i][sidx as usize][tidx as usize]
        }).fold(f64::INFINITY, f64::min);
        Some(m)
    }

    // Estimate the size of the memory footprint.
    pub fn estimate_size() -> usize {
        H * W * W * size_of::<u64>() + 2 * H * size_of::<u32>()
    }

    // Clear all counts by setting everything to zero.
    pub fn clear(&mut self) {
        for i in 0..H {
            for j in 0..W {
                self.sketch[i][j].fill(0.);
            }
        }
    }

    // Decay all counts by a factor of alpha.
    pub fn decay(&mut self, alpha: f64) {
        for i in 0..H {
            for j in 0..W {
                for k in 0..W {
                    self.sketch[i][j][k] *= alpha;
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hcms() {
        let mut hcms = HCountMin::<2, 10>::default();
        let source = "Hi";
        let target = "there";
        for _ in 0..100 {
            hcms.add(&source, &target);
        };
        assert_eq!(hcms.getcount(&source, &target), Some(100.));
        hcms.clear();
        assert_eq!(hcms.getcount(&source, &target), Some(0.));
        hcms.add(&source, &target);
        assert_eq!(hcms.getcount(&source, &target), Some(1.));
        hcms.decay(0.99);
        assert_eq!(hcms.getcount(&source, &target), Some(0.99));

        // We can seed the random hashes as well
        let mut s_hcms = HCountMin::<2, 10>::new(Some(42));
        for _ in 0..100 {
            s_hcms.add(&source, &target);
        };
        assert_eq!(s_hcms.getcount(&source, &target), Some(100.));
        s_hcms.clear();
        assert_eq!(s_hcms.getcount(&source, &target), Some(0.));
        s_hcms.add(&source, &target);
        assert_eq!(s_hcms.getcount(&source, &target), Some(1.));
        s_hcms.decay(0.99);
        assert_eq!(s_hcms.getcount(&source, &target), Some(0.99));
    }
}
