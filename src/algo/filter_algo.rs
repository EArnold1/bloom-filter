//! Bloom filter implementation utilities.
//!
//! This module provides a small Bloom filter implementation backed by a
//! `Vec<bool>` and MurmurHash3 (128-bit) hashing via `simplehash::murmurhash3_128`.
//!
//! The implementation includes a builder (`BloomFilterBuilder`) that derives
//! a recommended filter size and number of hash functions from an expected
//! item count and a desired false-positive probability.

use simplehash::murmurhash3_128;

/// Default expected number of items to be added to a filter when none is provided.
const DEFAULT_LIST_SIZE: usize = 30;

/// Builder for `BloomFilter`.
///
/// Use this to configure the expected number of elements and the acceptable
/// false-positive probability before constructing a `BloomFilter`.
pub struct BloomFilterBuilder {
    /// Desired false positive probability (p). Must be in the range [0, 1].
    false_positive_probability: f64,
    /// Expected number of items the filter will hold (n).
    list_size: usize,
}

/// A simple Bloom filter.
///
/// - `list` stores the bit array (represented as `Vec<bool>`).
/// - `hash_count` is the number of hash functions (k) used for insert/lookup.
/// - `size` is the size of the filter bit array (m).
pub struct BloomFilter {
    list: Vec<bool>,
    /// Number of hash functions used for this filter.
    hash_count: usize,
    /// Size of the underlying bit vector.
    size: usize,
}

impl Default for BloomFilterBuilder {
    fn default() -> Self {
        BloomFilterBuilder {
            false_positive_probability: 0.05,
            list_size: DEFAULT_LIST_SIZE,
        }
    }
}

impl BloomFilterBuilder {
    /// Create a new `BloomFilterBuilder`.
    ///
    /// # Panics
    ///
    /// Panics if `false_positive_probability` is not within `0.0..=1.0`.
    pub fn new(list_size: usize, false_positive_probability: f64) -> Self {
        assert!((0_f64..=1_f64).contains(&false_positive_probability),
                "false_positive_probability must be between 0 and 1");

        Self {
            false_positive_probability,
            list_size,
        }
    }
}

impl BloomFilterBuilder {
    /// Derive the optimal filter size `m` from `n` (expected item count) and `p` (false-positive probability).
    ///
    /// Uses the standard formula: m = -(n * ln(p)) / (ln(2)^2).
    fn derive_size(&self) -> usize {
        // m = -(n * ln(p)) / (ln(2)^2)
        let list_size: f64 = self.list_size as f64;
        let filter_size = -(list_size * self.false_positive_probability.ln()) / 2_f64.ln().powi(2);

        filter_size.round() as usize
    }

    /// Derive the optimal number of hash functions `k` for the given filter size `m`.
    ///
    /// Uses: k = (m/n) * ln(2).
    fn derive_hash_count(&self, filter_size: usize) -> usize {
        // k = (m/n) * ln(2)
        let hash_count = (filter_size as f64 / self.list_size as f64) * 2_f64.ln();

        hash_count.round() as usize
    }

    /// Build a `BloomFilter` from this builder's configuration.
    pub fn build(self) -> BloomFilter {
        let filter_size = self.derive_size();
        let hash_count = self.derive_hash_count(filter_size);
        BloomFilter {
            list: vec![false; filter_size],
            hash_count,
            size: filter_size,
        }
    }
}

impl Default for BloomFilter {
    fn default() -> Self {
        BloomFilterBuilder::default().build()
    }
}

impl BloomFilter {
    /// Insert an item into the bloom filter.
    ///
    /// The item is hashed `k` times with different seeds (0..k-1) using
    /// `murmurhash3_128`. Each resulting index sets the corresponding bit in
    /// the filter's bit vector.
    ///
    /// Note: the method currently accepts a `&str`; if you need arbitrary
    /// byte keys, convert them to bytes before calling or extend the API.
    pub fn insert(&mut self, item: &str) {
        for i in 0..self.hash_count {
            let seed = u32::try_from(i).expect("[Insert]: seed should be within u32 range");
            let digest = (murmurhash3_128(item.as_bytes(), seed) as usize) % self.size;

            self.list[digest] = true
        }
    }

    /// Check whether an item is possibly in the set.
    ///
    /// Returns `false` if the item is definitely not in the set; returns
    /// `true` if the item is possibly in the set (could be a false positive).
    pub fn lookup(&self, item: &str) -> bool {
        for i in 0..self.hash_count {
            let seed = u32::try_from(i).expect("[Check]: seed should be within u32 range");
            let digest = (murmurhash3_128(item.as_bytes(), seed) as usize) % self.size;

            if !self.list[digest] {
                return false;
            }
        }
        true
    }

    /// Return the number of hash functions used by this filter.
    pub fn get_hash_count(&self) -> usize {
        self.hash_count
    }

    /// Return the size of the underlying bit vector.
    pub fn get_size(&self) -> usize {
        self.size
    }
}
