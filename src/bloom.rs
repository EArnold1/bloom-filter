use simplehash::murmurhash3_128;

// Create traits for the `Filter`

pub struct Filter {
    list: Vec<bool>,
    size: usize,     // size of the list
    hash_count: u32, // number of hash functions
}

const DEFAULT_LIST_SIZE: usize = 16; // default size
const DEFAULT_HASH_COUNT: u32 = 3; // default size

impl Default for Filter {
    fn default() -> Self {
        let list = vec![false; DEFAULT_LIST_SIZE];
        println!("size {}", list.len());
        Filter {
            list,
            size: DEFAULT_LIST_SIZE,
            hash_count: DEFAULT_HASH_COUNT,
        }
    }
}

impl Filter {
    // TODO: Accept bytes
    pub fn insert(&mut self, item: &str) {
        for i in 0..self.hash_count {
            let digest = (murmurhash3_128(item.as_bytes(), i) as usize) % self.size;
            self.list[digest] = true
        }
    }

    pub fn check(&self, item: &str) -> bool {
        for i in 0..self.hash_count {
            let digest = (murmurhash3_128(item.as_bytes(), i) as usize) % self.size;

            if !self.list[digest] {
                return false;
            }
        }
        true
    }
}
