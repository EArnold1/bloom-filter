use crate::bloom::Filter;

pub mod bloom;

fn main() {
    let mut filter = Filter::default();

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

    let test_words = [word_present.clone(), word_absent.clone()].concat();

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
