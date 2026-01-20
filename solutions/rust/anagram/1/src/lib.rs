use std::collections::HashSet;

    pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
        let mut result = HashSet::new();

        let word_lower = word.to_lowercase();

        let mut word_chars:Vec<char> = word_lower.chars().collect();

        word_chars.sort_unstable();

        for &candidate in possible_anagrams{
            let candidate_lower = candidate.to_lowercase();
            if word_lower == candidate_lower{
                continue;
            }
            let mut chars:Vec<char> = candidate_lower.chars().collect();
            chars.sort_unstable();
            if word_chars == chars{
                result.insert(candidate);
            }
        }
        result
    }
