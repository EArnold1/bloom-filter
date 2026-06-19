use crate::bloom::FilterBuilder;

pub mod bloom;

fn main() {
    let mut filter = FilterBuilder::new(20, 0.1).build();

    println!(
        "filter size :{}, hash count :{}",
        &filter.filter_size, &filter.hash_count
    );

    let word_present = vec![
        "abound",
        "abounds",
        "abundance",
        "abundant",
        "accessible",
        "bloom",
        "blossom",
        "bolster",
        "bonny",
        "bonus",
        "bonuses",
        "coherent",
        "cohesive",
        "colorful",
        "comely",
        "comfort",
        "gems",
        "generosity",
        "generous",
        "generously",
        "genial",
    ];

    let word_absent = vec![
        "bluff",
        "cheater",
        "hate",
        "war",
        "humanity",
        "racism",
        "hurt",
        "nuke",
        "gloomy",
        "facebook",
        "geeksforgeeks",
        "twitter",
    ];

    for item in &word_present {
        filter.insert(item);
    }

    let test_words = [
        word_present.iter().take(10).cloned().collect(),
        word_absent.clone(),
    ]
    .concat();

    for word in test_words {
        if filter.check(word) {
            if word_absent.contains(&word) {
                println!("'{}' is a false positive!", &word)
            } else {
                println!("'{}' is probably present!", &word)
            }
        } else {
            println!("'{}' is definitely not present!", word)
        }
    }
}
