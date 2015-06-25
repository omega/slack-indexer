extern crate regex;
use self::regex::Regex;
#[allow(dead_code)]
pub fn get_bugs(text: String) -> Vec<String> {
    // TODO: Should be outside the function I think?
    let bug_hunter = Regex::new(r"\b([A-Z]+-\d+)\b").unwrap();

    let mut results: Vec<String> = Vec::new();
    for cap in bug_hunter.captures_iter(&text) {
        results.push(cap.at(1).unwrap_or("").to_string());
    }
    return results;
}

