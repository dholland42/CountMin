use std::hash::{DefaultHasher, Hash, Hasher};
use rand::prelude::*;
use rand::rngs::SmallRng;


#[derive(Debug)]
pub struct CountMin <const H: usize, const W: usize> {
    sketch: [[f64; W]; H],
    offsets_a: [u32; H],
    offsets_b: [u32; H],
}


impl<const H: usize, const W: usize> Default for CountMin <H, W> {
    /// Add a default implementation for CountMin
    ///
    /// # Examples
    ///
    /// ```
    /// use countmin::CountMin;
    ///
    /// let mut mc = CountMin::<2, 10>::default();
    /// mc.add(&"Hi");
    /// assert_eq!(mc.getcount(&"Hi"), Some(1.));
    /// ```
    fn default() -> Self {
        Self::new(None)
    }
}


impl <const H: usize, const W: usize> CountMin <H, W> {

    // Create a new MinCount sketch
    pub fn new(seed: Option<u64>) -> CountMin <H, W> {
        let mut rng = if let Some(s) = seed {
            SmallRng::seed_from_u64(s)
        } else {
            SmallRng::from_rng(&mut rand::rng())
        };
        let offsets_a: Vec<u32> = (0..H).map(|_| rng.random::<u32>() % (W as u32 - 1) + 1).collect();
        let offsets_b: Vec<u32> = (0..H).map(|_| rng.random::<u32>() % W as u32).collect();
        CountMin {
            sketch: [[0.; W]; H],
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
    pub fn add<T: Hash>(&mut self, item: &T) {
        let h = self.hash(item);
        self.add_idx(h);
    }

    pub fn add_idx(&mut self, h: u64) {
        for i in 0..H {
            // Note that we need to use wrapping arithmetic here
            let idx = (h).wrapping_mul(self.offsets_a[i] as u64).wrapping_add(self.offsets_b[i] as u64) % W as u64;
            self.sketch[i][idx as usize] += 1.;
        }
    }
    
    // Get the estimate for the number of times an item as heen seen so far by
    // hashing it into our buckets and then taking the minimum of all those values.
    // We use the minimum value as our estimate as we will only ever _over_count
    // items due to hash collisions.
    pub fn getcount<T: Hash>(&mut self, item: &T) -> Option<f64> {
        let h = self.hash(item);
        self.getcount_idx(h)
    }

    pub fn getcount_idx(&mut self, h: u64) -> Option<f64> {
        let m = (0..H).map(|i| {
            let idx = (h).wrapping_mul(self.offsets_a[i] as u64).wrapping_add(self.offsets_b[i] as u64) % W as u64;
            self.sketch[i][idx as usize]
        }).fold(f64::INFINITY, f64::min);
        Some(m)
    }

    // Estimate the size of the memory footprint.
    pub fn estimate_size() -> usize {
        H * W * size_of::<u64>() + 2 * H * size_of::<u32>()
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_mc() {
        let mut mc = CountMin::<2, 10>::default();
        let inp = "Hi";
        mc.add(&inp);
        assert_eq!(mc.getcount(&inp), Some(1.));

        // No collisions
        for _ in 0..100 {
            mc.add(&inp);
        }
        assert_eq!(mc.getcount(&inp), Some(101.));
    }
}
