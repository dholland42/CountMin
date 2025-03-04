use std::hash::{DefaultHasher, Hash, Hasher};
use rand::prelude::*;


#[derive(Debug)]
pub struct CountMin <const H: usize, const W: usize> {
    sketch: [[u64; W]; H],
    offsets_a: [u32; H],
    offsets_b: [u32; H],
}

// Add a `Default` implementation
impl<const H: usize, const W: usize> Default for CountMin <H, W> {
    fn default() -> Self {
        Self::new()
    }
}

// Implement the MinCount sketch
impl <const H: usize, const W: usize> CountMin <H, W> {

    // Create a new MinCount sketch
    pub fn new() -> CountMin <H, W> {
        let mut rng = rand::rng();
        let offsets_a: Vec<u32> = (0..H).map(|_| rng.random::<u32>() % (W as u32 - 1) + 1).collect();
        let offsets_b: Vec<u32> = (0..H).map(|_| rng.random::<u32>() % W as u32).collect();
        CountMin {
            sketch: [[0; W]; H],
            offsets_a: offsets_a.try_into().unwrap(),
            offsets_b: offsets_b.try_into().unwrap(),
        }
    }

    // A helper function to generate item hashes as u64
    fn hash<T: Hash>(&self, item: &T) -> u64 {
        let mut h = DefaultHasher::new();
        item.hash(&mut h);
        h.finish()
    }

    // Add an item to the sketch by hashing it into our buckets and incrementing
    // the count in each bucket.
    pub fn add<T: Hash>(&mut self, item: &T) {
        let h = self.hash(item);
        for i in 0..H {
            // Note that we need to use wrapping arithmetic here
            let idx = (h as u32).wrapping_mul(self.offsets_a[i]).wrapping_add(self.offsets_b[i]) % W as u32;
            self.sketch[i][idx as usize] += 1;
        }
    }
    
    // Get the estimate for the number of times an item as heen seen so far by
    // hashing it into our buckets and then taking the minimum of all those values.
    // We use the minimum value as our estimate as we will only ever _over_count
    // items due to hash collisions.
    pub fn getcount<T: Hash>(&mut self, item: &T) -> Option<u64> {
        let h = self.hash(item);
        (0..H).map(|i| {
            let idx = (h as u32).wrapping_mul(self.offsets_a[i]).wrapping_add(self.offsets_b[i]) % W as u32;
            self.sketch[i][idx as usize]
        }).min()
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
        let mut mc = CountMin::<2, 10>::new();
        let inp = "Hi";
        mc.add(&inp);
        assert_eq!(mc.getcount(&inp), Some(1));

        // No collisions
        for _ in 0..100 {
            mc.add(&inp);
        }
        assert_eq!(mc.getcount(&inp), Some(101));

        // This will not have been seen yet
        assert_eq!(mc.getcount(&"Not in the sketch"), Some(0));
    }
}
