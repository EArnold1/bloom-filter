# bloom_filter (Rust)

A small Rust implementation of a Bloom filter with a builder to derive
an appropriate bit-array size and number of hash functions from an
expected capacity and a target false-positive probability.

## Quick usage

Example snippet showing how to construct and use the filter from code:

```rust
use bloom_filter::BloomFilterBuilder;

let mut bf = BloomFilterBuilder::new(1000, 0.01).build();
bf.insert("alice@example.com");
assert!(bf.lookup("alice@example.com"));
assert!(!bf.lookup("bob@example.com"));
```

## Notes

- The implementation uses `simplehash::murmurhash3_128` for hashing.

### What is a Bloom Filter

A Bloom filter is a space-efficient **probabilistic** data structure that is used to test whether an element is a member of a set. For example, checking availability of a username is a set membership problem, where the set is the list of all registered usernames. The price we pay for efficiency is that it is **probabilistic** in nature that means, there might be some **_False Positive_** results.

### False Positive in Bloom Filters

The reason we say **"probably present"** is that Bloom filters can produce **false positives**. During a lookup, the hash functions may point to bits that were already set by other elements, causing the filter to indicate that an element exists even though it was never inserted.

**IMPORTANT**: If all the corresponding bits are set to `1`, the element is **probably present**. However, if any of those bits is `0`, the element is **definitely not present**. In other words, every hash function must point to a bit set to `1` for a lookup to indicate that the element may exist.

### References

1. [Geeksforgeeks](https://www.geeksforgeeks.org/python/bloom-filters-introduction-and-python-implementation/)
2. [Redis](https://redis.io/docs/latest/develop/data-types/probabilistic/bloom-filter/)
