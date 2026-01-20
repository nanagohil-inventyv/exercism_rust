use std::collections::HashSet;
use std::collections::HashMap;

    pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
        let mut result = HashSet::new();

        let word_lower = word.to_lowercase();
        let word_map = frec_map(&word_lower);

        for &candidate in possible_anagrams{
            let candidate_lower = candidate.to_lowercase();
            if word_lower == candidate_lower{
                continue;
            }

            if frec_map(&candidate_lower) == word_map{
                result.insert(candidate);
            }
        }
        result
    }

fn frec_map(s:&str)->HashMap<char,usize>{
    let mut map = HashMap::new();
    for c in s.chars(){
        *map.entry(c).or_insert(0) += 1;
    }
    map
}