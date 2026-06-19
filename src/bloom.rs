use simplehash::murmurhash3_128;

// Create traits for the `Filter`

// m = filter size, k = no. of hash functions, p = false positive probability, n = no. of items(expected capacity)
pub struct Filter {
    list: Vec<bool>,
    list_size: usize,       // Expected number of items to be stored in the filter
    pub hash_count: usize,  // The optimal number of hash functions
    pub filter_size: usize, // Bloom filter size
}

const DEFAULT_LIST_SIZE: usize = 30; // default size

pub struct FilterBuilder {
    false_positive_probability: f64, // [error rate] between 0 and 1
    list_size: usize,                // expected number of items to be stored in the filter
}

impl Default for FilterBuilder {
    fn default() -> Self {
        FilterBuilder {
            false_positive_probability: 0.05,
            list_size: DEFAULT_LIST_SIZE,
        }
    }
}

impl FilterBuilder {
    pub fn new(list_size: usize, false_positive_probability: f64) -> Self {
        assert!((0_f64..=1_f64).contains(&false_positive_probability));

        Self {
            false_positive_probability,
            list_size,
        }
    }
}

impl FilterBuilder {
    /// Size of the bloom filter in bits
    fn get_size(&self) -> usize {
        // m = -(n * math.log(p))/(math.log(2)**2)
        // where n = self.size, p = self.false_positive_probability
        let list_size: f64 = self.list_size as f64;
        let filter_size = -(list_size * self.false_positive_probability.ln()) / 2_f64.ln().powi(2);

        filter_size.round() as usize
    }

    fn get_hash_count(&self, filter_size: usize) -> usize {
        // k = (m/n) * lg(2)
        // where m = size of filter(self.filter_size), n = list_size
        let hash_count = (filter_size / self.list_size) as f64 * 2_f64.ln();

        hash_count.round() as usize
    }

    pub fn build(self) -> Filter {
        let filter_size = self.get_size();
        let hash_count = self.get_hash_count(filter_size);
        Filter {
            list: vec![false; self.list_size],
            list_size: self.list_size,
            hash_count,
            filter_size,
        }
    }
}

impl Default for Filter {
    fn default() -> Self {
        FilterBuilder::default().build()
    }
}

impl Filter {
    // TODO: Accept bytes
    pub fn insert(&mut self, item: &str) {
        let hash_count = self.hash_count;
        for i in 0..hash_count {
            let seed = u32::try_from(i).expect("[Insert]: seed should be within u32 range");
            let digest = (murmurhash3_128(item.as_bytes(), seed) as usize) % self.list_size;
            self.list[digest] = true
        }
    }

    pub fn check(&self, item: &str) -> bool {
        for i in 0..self.hash_count {
            let seed = u32::try_from(i).expect("[Check]: seed should be within u32 range");

            let digest = (murmurhash3_128(item.as_bytes(), seed) as usize) % self.list_size;

            if !self.list[digest] {
                return false;
            }
        }
        true
    }
}
