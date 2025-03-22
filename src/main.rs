use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashSet;

fn generate_planet_name() -> String {
    // Expanded lists of prefixes, middles, and suffixes
    let prefixes = [
        "Zor", "Xan", "Qua", "Lor", "Neb", "Vel", "Kry", "Aur", "Sol", "Dra", "Fyn", "Gax", "Hyl",
        "Jyn", "Kor", "Lux", "Myn", "Pax", "Ryn", "Tyx",
    ];
    let middles = ["ara", "ion", "eth", "une", "yx", "is", "on"];
    let suffixes = ["ion", "ara", "ius", "ora", "eth", "une", "yx", "is", "on"];

    // Create a random number generator
    let mut rng = rand::thread_rng();

    // Randomly select a prefix, middle, and suffix
    let prefix = prefixes.choose(&mut rng).unwrap_or(&"Unk");
    let middle = middles.choose(&mut rng).unwrap_or(&"def");
    let suffix = suffixes.choose(&mut rng).unwrap_or(&"alt");

    // Generate a random uppercase letter for the number prefix
    let alphabet_prefix = rng.gen_range(b'A'..=b'Z') as char;

    // Generate a random number for additional uniqueness
    let number = rng.gen_range(0..1_000_000_000); // Random number between 0 and 999,999,999

    // Combine the parts to form the planet name
    format!(
        "{}{}{} {}{}",
        prefix, middle, suffix, alphabet_prefix, number
    )
}

fn main() {
    // Use a HashSet to track unique planet names
    let mut planet_names = HashSet::new();
    let mut duplicates = Vec::new();

    // Generate and print 1,000,000,000 planet names
    for i in 1..=10 {
        let name = generate_planet_name();

        // Check for duplicates
        if !planet_names.insert(name.clone()) {
            duplicates.push((i, name.clone()));
        }

        println!("{}. {}", i, name);
    }

    // Print duplicate information
    if !duplicates.is_empty() {
        println!("\nDuplicates found:");
        for (index, name) in duplicates {
            println!("Duplicate at index {}: {}", index, name);
        }
    } else {
        println!("\nNo duplicates found.");
    }
}
