// M3: Seeded Random Number Generator
// Provides deterministic randomness for reproducible prompt generation

use std::num::Wrapping;

/// Seeded random number generator using Xorshift algorithm
/// Ensures deterministic prompt generation - same seed always produces same sequence
#[derive(Debug, Clone)]
pub struct SeededRandom {
    state: Wrapping<u64>,
}

impl SeededRandom {
    /// Create a new SeededRandom with the given seed
    /// Seed of 0 is converted to 1 to avoid degenerate state
    pub fn new(seed: u64) -> Self {
        let state = if seed == 0 {
            Wrapping(1u64)
        } else {
            Wrapping(seed)
        };

        let mut rng = SeededRandom { state };

        // Warm up the RNG - discard first few values
        // Xorshift can produce poor initial values
        for _ in 0..10 {
            rng.next_u64();
        }

        rng
    }

    /// Generate next random u64 using Xorshift64
    fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state.0
    }

    /// Generate random integer in range [min, max] (inclusive)
    #[allow(dead_code)] // Public API method for future use
    pub fn next_int(&mut self, min: i32, max: i32) -> i32 {
        if min > max {
            panic!("min must be <= max");
        }

        if min == max {
            return min;
        }

        let range = (max - min + 1) as u64;
        let value = self.next_u64() % range;
        min + value as i32
    }

    /// Generate random usize in range [0, max) (exclusive max)
    pub fn next_usize(&mut self, max: usize) -> usize {
        if max == 0 {
            return 0;
        }

        (self.next_u64() % max as u64) as usize
    }

    /// Generate random f32 in range [0.0, 1.0)
    pub fn next_f32(&mut self) -> f32 {
        // Use upper 53 bits and scale to [0, 1)
        // This is the standard approach used by rand crate
        let value = self.next_u64();
        let scaled = (value >> 11) as f64; // Use 53 bits
        (scaled / 9007199254740992.0) as f32 // 2^53
    }

    /// Select random index from weighted items
    /// Weights are normalized internally
    pub fn weighted_choice(&mut self, weights: &[f32]) -> usize {
        if weights.is_empty() {
            panic!("Cannot choose from empty weights");
        }

        if weights.len() == 1 {
            return 0;
        }

        // Calculate total weight
        let total: f32 = weights.iter().sum();

        if total <= 0.0 {
            // All weights are 0 or negative, choose uniformly
            return self.next_usize(weights.len());
        }

        // Generate random value in [0, total)
        let mut target = self.next_f32() * total;

        // Find the weighted index
        for (i, &weight) in weights.iter().enumerate() {
            target -= weight;
            if target <= 0.0 {
                return i;
            }
        }

        // Fallback (shouldn't happen due to floating point precision)
        weights.len() - 1
    }

    /// M9: Select random index from weighted items (f64 version for rulebooks)
    /// Weights are normalized internally
    pub fn weighted_choice_f64(&mut self, weights: &[f64]) -> usize {
        if weights.is_empty() {
            panic!("Cannot choose from empty weights");
        }

        if weights.len() == 1 {
            return 0;
        }

        // Calculate total weight
        let total: f64 = weights.iter().sum();

        if total <= 0.0 {
            // All weights are 0 or negative, choose uniformly
            return self.next_usize(weights.len());
        }

        // Generate random value in [0, total)
        let mut target = self.next_f32() as f64 * total;

        // Find the weighted index
        for (i, &weight) in weights.iter().enumerate() {
            target -= weight;
            if target <= 0.0 {
                return i;
            }
        }

        // Fallback (shouldn't happen due to floating point precision)
        weights.len() - 1
    }

    /// M5 Phase 3+4: Generate random value in range [min, max] (inclusive)
    pub fn gen_range(&mut self, range: std::ops::RangeInclusive<usize>) -> usize {
        let min = *range.start();
        let max = *range.end();

        if min > max {
            panic!("min must be <= max");
        }

        if min == max {
            return min;
        }

        let range_size = (max - min + 1) as u64;
        let value = self.next_u64() % range_size;
        min + value as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determinism() {
        let mut rng1 = SeededRandom::new(42);
        let mut rng2 = SeededRandom::new(42);

        // Same seed should produce same sequence
        for _ in 0..100 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn test_different_seeds_produce_different_sequences() {
        let mut rng1 = SeededRandom::new(42);
        let mut rng2 = SeededRandom::new(43);

        // Different seeds should produce different values
        let v1 = rng1.next_u64();
        let v2 = rng2.next_u64();
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_next_int_range() {
        let mut rng = SeededRandom::new(12345);

        for _ in 0..100 {
            let val = rng.next_int(1, 10);
            assert!(val >= 1 && val <= 10);
        }
    }

    #[test]
    fn test_next_int_single_value() {
        let mut rng = SeededRandom::new(12345);
        assert_eq!(rng.next_int(5, 5), 5);
    }

    #[test]
    fn test_next_usize() {
        let mut rng = SeededRandom::new(67890);

        for _ in 0..100 {
            let val = rng.next_usize(10);
            assert!(val < 10);
        }
    }

    #[test]
    fn test_next_f32_range() {
        let mut rng = SeededRandom::new(11111);

        for _ in 0..100 {
            let val = rng.next_f32();
            assert!(val >= 0.0 && val < 1.0);
        }
    }

    #[test]
    fn test_weighted_choice_equal_weights() {
        let mut rng = SeededRandom::new(99999);
        let weights = vec![1.0, 1.0, 1.0, 1.0];

        // Should distribute roughly evenly
        let mut counts = vec![0; 4];
        for _ in 0..1000 {
            let idx = rng.weighted_choice(&weights);
            counts[idx] += 1;
        }

        // Each should be chosen roughly 250 times (allow for variance)
        for count in counts {
            assert!(count > 150 && count < 350);
        }
    }

    #[test]
    fn test_weighted_choice_skewed_weights() {
        let mut rng = SeededRandom::new(55555);
        let weights = vec![9.0, 1.0]; // 90% vs 10%

        let mut counts = vec![0; 2];
        for _ in 0..1000 {
            let idx = rng.weighted_choice(&weights);
            counts[idx] += 1;
        }

        // First should be chosen ~90% of the time
        assert!(counts[0] > 800 && counts[0] < 950);
        assert!(counts[1] > 50 && counts[1] < 200);
    }

    #[test]
    fn test_weighted_choice_single_item() {
        let mut rng = SeededRandom::new(77777);
        let weights = vec![5.0];
        assert_eq!(rng.weighted_choice(&weights), 0);
    }

    #[test]
    fn test_seed_zero_handled() {
        let mut rng = SeededRandom::new(0);
        // Should not crash and should produce values
        let _ = rng.next_u64();
    }
}
