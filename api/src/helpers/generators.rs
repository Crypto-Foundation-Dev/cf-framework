use rand::distr::Alphanumeric;
use rand::Rng;

// Create news slug by title and add random unique behind
// Ex title: Alindra Putra Programmer Terbaik Seluruh Depok
// Generated: alindra-putra-programmer-terbaik-seluruh-depok-a1b2c3
pub fn make_slug(title: &str) -> String {
    let base = title
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric(), "-") // replace non-alphanumeric with '-'
        .split('-')
        .filter(|s| !s.is_empty()) // remove empty parts
        .collect::<Vec<_>>()
        .join("-");

    let suffix: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect::<String>()
        .to_lowercase();

    format!("{}-{}", base, suffix)
}