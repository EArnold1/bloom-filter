use simplehash::murmurhash3_128;

// default number of items to add
const DEFAULT_LIST_SIZE: usize = 30;

// Create traits for the `BloomFilter`

pub struct BloomFilterBuilder {
    false_positive_probability: f64, // [error rate] between 0 and 1
    list_size: usize,                // expected number of items to be stored in the filter
}

// m = filter size, k = no. of hash functions, p = false positive probability, n = no. of items(expected capacity)
pub struct BloomFilter {
    list: Vec<bool>,
    hash_count: usize, // The optimal number of hash functions
    size: usize,       // Bloom filter size
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
    pub fn new(list_size: usize, false_positive_probability: f64) -> Self {
        assert!((0_f64..=1_f64).contains(&false_positive_probability));

        Self {
            false_positive_probability,
            list_size,
        }
    }
}

impl BloomFilterBuilder {
    /// Size of the bloom filter
    fn derive_size(&self) -> usize {
        // m = -(n * math.log(p))/(math.log(2)**2)
        // where n = self.size, p = self.false_positive_probability
        let list_size: f64 = self.list_size as f64;
        let filter_size = -(list_size * self.false_positive_probability.ln()) / 2_f64.ln().powi(2);

        filter_size.round() as usize
    }

    fn derive_hash_count(&self, filter_size: usize) -> usize {
        // k = (m/n) * lg(2)
        // where m = size of filter(self.filter_size), n = list_size
        let hash_count = (filter_size / self.list_size) as f64 * 2_f64.ln();

        hash_count.round() as usize
    }

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
    // TODO: Accept bytes
    pub fn insert(&mut self, item: &str) {
        for i in 0..self.hash_count {
            let seed = u32::try_from(i).expect("[Insert]: seed should be within u32 range");
            let digest = (murmurhash3_128(item.as_bytes(), seed) as usize) % self.size;

            self.list[digest] = true
        }
    }

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

    pub fn get_hash_count(&self) -> usize {
        self.hash_count
    }
    pub fn get_size(&self) -> usize {
        self.size
    }
}
